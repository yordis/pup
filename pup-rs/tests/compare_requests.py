#!/usr/bin/env python3
"""
compare_requests.py -- Compare Go and Rust pup request logs for API parity.

Usage:
    python3 compare_requests.py <go_log.jsonl> <rust_log.jsonl>

Parses both JSONL request logs produced by the mock server, canonicalises
every request, and reports on differences.

Exit codes:
    0 -- full parity
    1 -- gaps remain
"""

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

# ---------------------------------------------------------------------------
# Canonicalisation
# ---------------------------------------------------------------------------

# Patterns that look like IDs in URL path segments.
_UUID_RE = re.compile(
    r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}"
)
_NUMERIC_RE = re.compile(r"^\d+$")
_HEX_ID_RE = re.compile(r"^[0-9a-fA-F]{12,}$")
_SLUG_ID_RE = re.compile(r"^[a-z0-9]+-[a-z0-9]+-[a-z0-9]+")  # e.g. abc-def-ghi


def canonicalise_path(path: str) -> str:
    """Replace ID-like path segments with {id} for grouping."""
    parts = path.strip("/").split("/")
    out = []
    for part in parts:
        if _UUID_RE.fullmatch(part):
            out.append("{id}")
        elif _NUMERIC_RE.fullmatch(part):
            out.append("{id}")
        elif _HEX_ID_RE.fullmatch(part):
            out.append("{id}")
        elif _SLUG_ID_RE.fullmatch(part) and len(part) > 6:
            # Only replace slug-like segments that are long enough to be IDs
            # but skip known API path segments.
            known_segments = {
                "api", "v1", "v2", "monitor", "dashboard", "logs", "events",
                "search", "aggregate", "incidents", "query", "metrics",
                "rum", "applications", "team", "ip_ranges", "validate",
                "downtime", "slo", "correction", "synthetics", "tests",
                "trigger", "results", "notebooks", "hosts", "totals",
                "users", "roles", "service_accounts", "api_keys",
                "application_keys", "security_monitoring", "rules",
                "signals", "sensitive-data-scanner", "config", "cost",
                "enabled", "estimated", "series", "dashboard_lists",
                "manual", "dashboards", "mute", "unmute", "host",
            }
            if part.lower() in known_segments:
                out.append(part)
            else:
                out.append("{id}")
        else:
            out.append(part)
    return "/" + "/".join(out)


def canonicalise_request(entry: dict) -> str:
    """Return a canonical string like 'GET /api/v1/monitor'."""
    method = entry.get("method", "?").upper()
    path = canonicalise_path(entry.get("path", "/"))
    return f"{method} {path}"


# ---------------------------------------------------------------------------
# Parsing
# ---------------------------------------------------------------------------

def parse_log(path: str) -> list:
    """Read a JSONL file and return a list of parsed entries."""
    entries = []
    p = Path(path)
    if not p.exists():
        print(f"WARNING: log file not found: {path}", file=sys.stderr)
        return entries
    for line_no, line in enumerate(p.read_text().splitlines(), 1):
        line = line.strip()
        if not line:
            continue
        try:
            entries.append(json.loads(line))
        except json.JSONDecodeError as e:
            print(f"WARNING: invalid JSON at {path}:{line_no}: {e}", file=sys.stderr)
    return entries


# ---------------------------------------------------------------------------
# Analysis
# ---------------------------------------------------------------------------

def analyse_auth(go_entries: list, rust_entries: list) -> list:
    """Check for auth header differences."""
    diffs = []

    def has_auth(entries: list) -> dict:
        """Return a dict of canonical_key -> set of auth header states."""
        result = defaultdict(set)
        for e in entries:
            key = canonicalise_request(e)
            headers = e.get("headers", {})
            api_key = headers.get("DD-API-KEY", headers.get("Dd-Api-Key",
                       headers.get("dd-api-key", "absent")))
            auth = headers.get("Authorization", headers.get("authorization", "absent"))
            result[key].add(f"api_key={api_key},auth={auth}")
        return result

    go_auth = has_auth(go_entries)
    rust_auth = has_auth(rust_entries)

    common_keys = set(go_auth.keys()) & set(rust_auth.keys())
    for key in sorted(common_keys):
        go_states = go_auth[key]
        rust_states = rust_auth[key]
        if go_states != rust_states:
            diffs.append({
                "endpoint": key,
                "go_auth": sorted(go_states),
                "rust_auth": sorted(rust_states),
            })
    return diffs


def analyse_body_shapes(go_entries: list, rust_entries: list) -> list:
    """Compare request body structures for shared endpoints."""
    diffs = []

    def body_shapes(entries: list) -> dict:
        """Return dict of canonical_key -> set of body top-level key tuples."""
        result = defaultdict(set)
        for e in entries:
            key = canonicalise_request(e)
            body = e.get("body", "")
            if not body:
                result[key].add(("__empty__",))
                continue
            try:
                parsed = json.loads(body)
                if isinstance(parsed, dict):
                    result[key].add(tuple(sorted(parsed.keys())))
                else:
                    result[key].add(("__non_dict__",))
            except (json.JSONDecodeError, TypeError):
                result[key].add(("__unparseable__",))
        return result

    go_shapes = body_shapes(go_entries)
    rust_shapes = body_shapes(rust_entries)

    common_keys = set(go_shapes.keys()) & set(rust_shapes.keys())
    for key in sorted(common_keys):
        go_s = go_shapes[key]
        rust_s = rust_shapes[key]
        if go_s != rust_s:
            diffs.append({
                "endpoint": key,
                "go_body_keys": [list(t) for t in sorted(go_s)],
                "rust_body_keys": [list(t) for t in sorted(rust_s)],
            })
    return diffs


# ---------------------------------------------------------------------------
# Reporting
# ---------------------------------------------------------------------------

def print_section(title: str, char: str = "="):
    width = max(60, len(title) + 4)
    print(f"\n{char * width}")
    print(f"  {title}")
    print(f"{char * width}")


def main():
    if len(sys.argv) < 3:
        print(f"Usage: {sys.argv[0]} <go_log.jsonl> <rust_log.jsonl>", file=sys.stderr)
        sys.exit(2)

    go_path = sys.argv[1]
    rust_path = sys.argv[2]

    go_entries = parse_log(go_path)
    rust_entries = parse_log(rust_path)

    print_section("Request Log Comparison Report")
    print(f"  Go log:   {go_path} ({len(go_entries)} requests)")
    print(f"  Rust log: {rust_path} ({len(rust_entries)} requests)")

    # Canonicalise and count.
    go_endpoints = Counter(canonicalise_request(e) for e in go_entries)
    rust_endpoints = Counter(canonicalise_request(e) for e in rust_entries)

    go_set = set(go_endpoints.keys())
    rust_set = set(rust_endpoints.keys())
    common = go_set & rust_set
    go_only = go_set - rust_set
    rust_only = rust_set - go_set

    # -- Endpoint coverage -------------------------------------------------
    print_section("Endpoint Coverage", "-")
    print(f"  Go endpoints:     {len(go_set)}")
    print(f"  Rust endpoints:   {len(rust_set)}")
    print(f"  Common:           {len(common)}")
    print(f"  Go only:          {len(go_only)}")
    print(f"  Rust only:        {len(rust_only)}")

    has_gaps = False

    if go_only:
        has_gaps = True
        print_section("Endpoints in Go but NOT in Rust (missing in Rust)", "-")
        for ep in sorted(go_only):
            print(f"  MISSING  {ep}  (Go called {go_endpoints[ep]}x)")

    if rust_only:
        # Rust-only endpoints are not necessarily gaps; they may be extras.
        print_section("Endpoints in Rust but NOT in Go (extra in Rust)", "-")
        for ep in sorted(rust_only):
            print(f"  EXTRA    {ep}  (Rust called {rust_endpoints[ep]}x)")

    # -- Auth header differences -------------------------------------------
    auth_diffs = analyse_auth(go_entries, rust_entries)
    if auth_diffs:
        has_gaps = True
        print_section("Auth Header Differences", "-")
        for d in auth_diffs:
            print(f"  {d['endpoint']}")
            print(f"    Go:   {d['go_auth']}")
            print(f"    Rust: {d['rust_auth']}")

    # -- Body structure differences ----------------------------------------
    body_diffs = analyse_body_shapes(go_entries, rust_entries)
    if body_diffs:
        has_gaps = True
        print_section("Request Body Structure Differences", "-")
        for d in body_diffs:
            print(f"  {d['endpoint']}")
            print(f"    Go keys:   {d['go_body_keys']}")
            print(f"    Rust keys: {d['rust_body_keys']}")

    # -- Shared endpoint call counts ---------------------------------------
    if common:
        print_section("Shared Endpoint Call Counts", "-")
        print(f"  {'Endpoint':<50} {'Go':>6} {'Rust':>6}")
        print(f"  {'-'*50} {'-'*6} {'-'*6}")
        for ep in sorted(common):
            go_n = go_endpoints[ep]
            rust_n = rust_endpoints[ep]
            marker = "" if go_n == rust_n else "  <-- diff"
            print(f"  {ep:<50} {go_n:>6} {rust_n:>6}{marker}")

    # -- Summary -----------------------------------------------------------
    print_section("Summary")
    total_go = len(go_set)
    total_rust = len(rust_set)
    if total_go > 0:
        coverage_pct = len(common) / total_go * 100
    else:
        coverage_pct = 100.0 if total_rust == 0 else 0.0

    print(f"  Rust covers {len(common)}/{total_go} Go endpoints ({coverage_pct:.1f}%)")
    print(f"  Missing endpoints: {len(go_only)}")
    print(f"  Extra endpoints:   {len(rust_only)}")
    print(f"  Auth diffs:        {len(auth_diffs)}")
    print(f"  Body diffs:        {len(body_diffs)}")

    if not has_gaps and len(go_only) == 0:
        print(f"\n  RESULT: PASS -- Full API parity")
        return 0
    else:
        print(f"\n  RESULT: FAIL -- Gaps remain")
        return 1


if __name__ == "__main__":
    sys.exit(main())
