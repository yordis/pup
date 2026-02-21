#!/usr/bin/env python3
"""
Mock Datadog API server for pup / pup-rs comparison testing.

Uses only the Python standard library.  Listens on the port given as the first
CLI argument (default 19876) and serves canned fixture responses so that both
the Go and Rust CLIs can be exercised without hitting the real Datadog API.

Every inbound request is logged as a JSON-lines entry to
/tmp/pup_mock_requests.jsonl so the comparison harness can diff behaviour.
"""

import http.server
import json
import os
import re
import sys
import urllib.parse
from pathlib import Path

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------

DEFAULT_PORT = 19876
LOG_PATH = "/tmp/pup_mock_requests.jsonl"
FIXTURE_DIR = Path(__file__).resolve().parent / "fixtures"

# Headers whose *values* we never log -- we only record present / absent.
SENSITIVE_HEADERS = {"dd-api-key", "dd-application-key", "authorization"}

# ---------------------------------------------------------------------------
# Route table
#
# Each entry is (HTTP method, regex compiled from a path pattern, fixture file).
# Path patterns use {id} as a wildcard that matches a single path segment
# (UUIDs, integers, slugs, etc.).
# ---------------------------------------------------------------------------

_ID = r"[^/]+"  # matches one path segment

ROUTE_TABLE_SRC = [
    # --- Monitors (v1) ---
    ("GET",    "/api/v1/monitor",             "v1_monitors.json"),
    ("POST",   "/api/v1/monitor",             "v1_monitor.json"),
    ("GET",    "/api/v1/monitor/{id}",        "v1_monitor.json"),
    ("PUT",    "/api/v1/monitor/{id}",        "v1_monitor.json"),
    ("DELETE", "/api/v1/monitor/{id}",        "v1_deleted.json"),

    # --- Dashboards (v1) ---
    ("GET",    "/api/v1/dashboard",           "v1_dashboards.json"),
    ("POST",   "/api/v1/dashboard",           "v1_dashboard.json"),
    ("GET",    "/api/v1/dashboard/{id}",      "v1_dashboard.json"),
    ("PUT",    "/api/v1/dashboard/{id}",      "v1_dashboard.json"),
    ("DELETE", "/api/v1/dashboard/{id}",      "v1_deleted.json"),

    # --- Dashboard Lists (v1 / v2) ---
    ("GET",    "/api/v1/dashboard/lists/manual",             "v1_dashboards.json"),
    ("POST",   "/api/v1/dashboard/lists/manual",             "v1_dashboard.json"),
    ("GET",    "/api/v1/dashboard/lists/manual/{id}",        "v1_dashboard.json"),
    ("PUT",    "/api/v1/dashboard/lists/manual/{id}",        "v1_dashboard.json"),
    ("DELETE", "/api/v1/dashboard/lists/manual/{id}",        "v1_deleted.json"),
    ("GET",    "/api/v2/dashboard/lists/manual/{id}/dashboards", "v1_dashboards.json"),

    # --- Logs (v2) ---
    ("POST",   "/api/v2/logs/events/search",     "v2_logs_list.json"),
    ("POST",   "/api/v2/logs/analytics/aggregate", "v2_logs_aggregate.json"),

    # --- Incidents (v2) ---
    ("GET",    "/api/v2/incidents",           "v2_incidents.json"),
    ("POST",   "/api/v2/incidents",           "v2_incident.json"),
    ("GET",    "/api/v2/incidents/{id}",      "v2_incident.json"),
    ("PATCH",  "/api/v2/incidents/{id}",      "v2_incident.json"),
    ("DELETE", "/api/v2/incidents/{id}",      "v2_ok.json"),

    # --- Metrics (v1 / v2) ---
    ("GET",    "/api/v1/query",               "v1_metrics.json"),
    ("GET",    "/api/v1/metrics",             "v1_metrics.json"),
    ("GET",    "/api/v1/metrics/{id}",        "v1_metrics.json"),
    ("POST",   "/api/v2/series",              "v2_ok.json"),

    # --- RUM (v2) ---
    ("GET",    "/api/v2/rum/applications",    "v2_rum_apps.json"),
    ("POST",   "/api/v2/rum/applications",    "v2_rum_apps.json"),
    ("GET",    "/api/v2/rum/applications/{id}", "v2_rum_apps.json"),
    ("PUT",    "/api/v2/rum/applications/{id}", "v2_rum_apps.json"),
    ("DELETE", "/api/v2/rum/applications/{id}", "v2_ok.json"),
    ("POST",   "/api/v2/rum/events/search",   "v2_logs_list.json"),

    # --- Teams (v2) ---
    ("GET",    "/api/v2/team",                "v2_teams.json"),
    ("POST",   "/api/v2/team",                "v2_teams.json"),
    ("GET",    "/api/v2/team/{id}",           "v2_teams.json"),
    ("PATCH",  "/api/v2/team/{id}",           "v2_teams.json"),
    ("DELETE", "/api/v2/team/{id}",           "v2_ok.json"),

    # --- IP Ranges (v1) ---
    ("GET",    "/api/v1/ip_ranges",           "v1_ip_ranges.json"),

    # --- Downtimes (v1 / v2) ---
    ("GET",    "/api/v1/downtime",            "v1_monitors.json"),
    ("POST",   "/api/v1/downtime",            "v1_monitor.json"),
    ("GET",    "/api/v1/downtime/{id}",       "v1_monitor.json"),
    ("PUT",    "/api/v1/downtime/{id}",       "v1_monitor.json"),
    ("DELETE", "/api/v1/downtime/{id}",       "v1_deleted.json"),
    ("GET",    "/api/v2/downtime",            "v2_incidents.json"),
    ("POST",   "/api/v2/downtime",            "v2_incident.json"),
    ("GET",    "/api/v2/downtime/{id}",       "v2_incident.json"),
    ("PATCH",  "/api/v2/downtime/{id}",       "v2_incident.json"),
    ("DELETE", "/api/v2/downtime/{id}",       "v2_ok.json"),

    # --- SLOs (v1) ---
    ("GET",    "/api/v1/slo",                 "v1_monitors.json"),
    ("POST",   "/api/v1/slo",                 "v1_monitor.json"),
    ("GET",    "/api/v1/slo/{id}",            "v1_monitor.json"),
    ("PUT",    "/api/v1/slo/{id}",            "v1_monitor.json"),
    ("DELETE", "/api/v1/slo/{id}",            "v1_deleted.json"),

    # --- Synthetics (v1) ---
    ("GET",    "/api/v1/synthetics/tests",    "v1_monitors.json"),
    ("POST",   "/api/v1/synthetics/tests",    "v1_monitor.json"),
    ("GET",    "/api/v1/synthetics/tests/{id}", "v1_monitor.json"),
    ("PUT",    "/api/v1/synthetics/tests/{id}", "v1_monitor.json"),
    ("DELETE", "/api/v1/synthetics/tests/{id}", "v1_deleted.json"),
    ("POST",   "/api/v1/synthetics/tests/trigger", "v2_ok.json"),
    ("GET",    "/api/v1/synthetics/tests/{id}/results", "v1_monitors.json"),

    # --- Notebooks (v1) ---
    ("GET",    "/api/v1/notebooks",           "v1_monitors.json"),
    ("POST",   "/api/v1/notebooks",           "v1_monitor.json"),
    ("GET",    "/api/v1/notebooks/{id}",      "v1_monitor.json"),
    ("PUT",    "/api/v1/notebooks/{id}",      "v1_monitor.json"),
    ("DELETE", "/api/v1/notebooks/{id}",      "v1_deleted.json"),

    # --- Service Level Corrections (v1) ---
    ("GET",    "/api/v1/slo/correction",              "v1_monitors.json"),
    ("POST",   "/api/v1/slo/correction",              "v1_monitor.json"),
    ("GET",    "/api/v1/slo/correction/{id}",         "v1_monitor.json"),
    ("PATCH",  "/api/v1/slo/correction/{id}",         "v1_monitor.json"),
    ("DELETE", "/api/v1/slo/correction/{id}",         "v1_deleted.json"),

    # --- Events (v1 / v2) ---
    ("GET",    "/api/v1/events",              "v1_monitors.json"),
    ("POST",   "/api/v1/events",              "v1_monitor.json"),
    ("GET",    "/api/v1/events/{id}",         "v1_monitor.json"),
    ("POST",   "/api/v2/events/search",       "v2_logs_list.json"),

    # --- Hosts (v1) ---
    ("GET",    "/api/v1/hosts",               "v1_monitors.json"),
    ("GET",    "/api/v1/hosts/totals",        "v1_monitor.json"),
    ("POST",   "/api/v1/host/mute/{id}",      "v2_ok.json"),
    ("POST",   "/api/v1/host/unmute/{id}",     "v2_ok.json"),

    # --- Users (v2) ---
    ("GET",    "/api/v2/users",               "v2_teams.json"),
    ("POST",   "/api/v2/users",               "v2_teams.json"),
    ("GET",    "/api/v2/users/{id}",          "v2_teams.json"),
    ("PATCH",  "/api/v2/users/{id}",          "v2_teams.json"),
    ("DELETE", "/api/v2/users/{id}",          "v2_ok.json"),

    # --- Roles (v2) ---
    ("GET",    "/api/v2/roles",               "v2_teams.json"),
    ("POST",   "/api/v2/roles",               "v2_teams.json"),
    ("GET",    "/api/v2/roles/{id}",          "v2_teams.json"),
    ("PATCH",  "/api/v2/roles/{id}",          "v2_teams.json"),
    ("DELETE", "/api/v2/roles/{id}",          "v2_ok.json"),

    # --- Service Accounts (v2) ---
    ("GET",    "/api/v2/service_accounts",    "v2_teams.json"),
    ("POST",   "/api/v2/service_accounts",    "v2_teams.json"),
    ("GET",    "/api/v2/service_accounts/{id}", "v2_teams.json"),
    ("PATCH",  "/api/v2/service_accounts/{id}", "v2_teams.json"),
    ("DELETE", "/api/v2/service_accounts/{id}", "v2_ok.json"),

    # --- API / App Keys (v2) ---
    ("GET",    "/api/v2/api_keys",            "v2_teams.json"),
    ("POST",   "/api/v2/api_keys",            "v2_teams.json"),
    ("GET",    "/api/v2/api_keys/{id}",       "v2_teams.json"),
    ("PATCH",  "/api/v2/api_keys/{id}",       "v2_teams.json"),
    ("DELETE", "/api/v2/api_keys/{id}",       "v2_ok.json"),
    ("GET",    "/api/v2/application_keys",    "v2_teams.json"),
    ("POST",   "/api/v2/application_keys",    "v2_teams.json"),
    ("GET",    "/api/v2/application_keys/{id}", "v2_teams.json"),
    ("PATCH",  "/api/v2/application_keys/{id}", "v2_teams.json"),
    ("DELETE", "/api/v2/application_keys/{id}", "v2_ok.json"),

    # --- Security (v2) ---
    ("GET",    "/api/v2/security_monitoring/rules",       "v2_incidents.json"),
    ("POST",   "/api/v2/security_monitoring/rules",       "v2_incident.json"),
    ("GET",    "/api/v2/security_monitoring/rules/{id}",  "v2_incident.json"),
    ("PUT",    "/api/v2/security_monitoring/rules/{id}",  "v2_incident.json"),
    ("DELETE", "/api/v2/security_monitoring/rules/{id}",  "v2_ok.json"),
    ("GET",    "/api/v2/security_monitoring/signals",     "v2_incidents.json"),
    ("POST",   "/api/v2/security_monitoring/signals/search", "v2_incidents.json"),

    # --- Sensitive Data Scanner (v2) ---
    ("GET",    "/api/v2/sensitive-data-scanner/config",   "v2_incidents.json"),

    # --- Cloud Cost (v2) ---
    ("GET",    "/api/v2/cost/enabled",                    "v2_ok.json"),
    ("GET",    "/api/v2/cost/estimated",                  "v1_metrics.json"),

    # --- Catch-all for validate endpoint ---
    ("GET",    "/api/v1/validate",            "v2_ok.json"),
]

# Compile the route table: convert {id} patterns to regex.
ROUTES = []
for method, pattern, fixture_file in ROUTE_TABLE_SRC:
    # Escape dots/slashes, then replace {id} with segment matcher
    regex_str = "^" + re.escape(pattern).replace(r"\{id\}", _ID) + "$"
    ROUTES.append((method.upper(), re.compile(regex_str), fixture_file))


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def load_fixture(name: str) -> bytes:
    """Load a fixture file and return its bytes."""
    path = FIXTURE_DIR / name
    if not path.exists():
        return json.dumps({"errors": [f"fixture {name} not found"]}).encode()
    return path.read_bytes()


def sanitize_headers(headers: dict) -> dict:
    """Replace sensitive header values with present/absent markers."""
    out = {}
    for key, value in headers.items():
        lower = key.lower()
        if lower in SENSITIVE_HEADERS:
            out[key] = "present" if value else "absent"
        else:
            out[key] = value
    return out


def log_request(entry: dict) -> None:
    """Append a JSON-lines entry to the request log."""
    with open(LOG_PATH, "a") as f:
        f.write(json.dumps(entry, separators=(",", ":")) + "\n")


def match_route(method: str, path: str):
    """Find the first matching route and return its fixture filename, or None."""
    for route_method, route_re, fixture_file in ROUTES:
        if method == route_method and route_re.match(path):
            return fixture_file
    return None


# ---------------------------------------------------------------------------
# Request handler
# ---------------------------------------------------------------------------

class MockDDHandler(http.server.BaseHTTPRequestHandler):
    """Handle all HTTP methods uniformly."""

    # Suppress per-request logging to stderr (we log to the JSONL file).
    def log_message(self, format, *args):
        pass

    def _handle(self):
        # Parse the URL.
        parsed = urllib.parse.urlparse(self.path)
        path = parsed.path.rstrip("/") or "/"
        query = parsed.query
        method = self.command.upper()

        # Read body if present.
        content_length = int(self.headers.get("Content-Length", 0))
        body = ""
        if content_length > 0:
            raw = self.rfile.read(content_length)
            try:
                body = raw.decode("utf-8")
            except UnicodeDecodeError:
                body = "<binary>"

        # Collect headers as a plain dict.
        raw_headers = {k: v for k, v in self.headers.items()}

        # Log the request.
        log_entry = {
            "method": method,
            "path": path,
            "query": query,
            "headers": sanitize_headers(raw_headers),
            "body": body,
        }
        log_request(log_entry)

        # Try to match a route.
        fixture_file = match_route(method, path)
        if fixture_file:
            data = load_fixture(fixture_file)
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(data)))
            self.end_headers()
            self.wfile.write(data)
        else:
            body_404 = json.dumps({"errors": ["not found"]}).encode()
            self.send_response(404)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(body_404)))
            self.end_headers()
            self.wfile.write(body_404)

    # Route every method through the same handler.
    def do_GET(self):
        self._handle()

    def do_POST(self):
        self._handle()

    def do_PUT(self):
        self._handle()

    def do_PATCH(self):
        self._handle()

    def do_DELETE(self):
        self._handle()

    def do_HEAD(self):
        self._handle()

    def do_OPTIONS(self):
        self._handle()


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else DEFAULT_PORT

    # Truncate the log file on startup.
    with open(LOG_PATH, "w"):
        pass

    http.server.HTTPServer.allow_reuse_address = True
    server = http.server.HTTPServer(("127.0.0.1", port), MockDDHandler)
    server.allow_reuse_address = True
    print(f"Mock Datadog API server listening on http://127.0.0.1:{port}", flush=True)
    print(f"Request log: {LOG_PATH}", flush=True)
    print(f"Fixture dir: {FIXTURE_DIR}", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down mock server.", flush=True)
    finally:
        server.server_close()


if __name__ == "__main__":
    main()
