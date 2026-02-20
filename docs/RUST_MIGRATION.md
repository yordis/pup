# Rust Migration Plan

Migration plan for rewriting the `pup` CLI from Go to Rust.

**Status:** Planning
**Last updated:** 2026-02-20

---

## Table of Contents

- [Overview](#overview)
- [Current Codebase Snapshot](#current-codebase-snapshot)
- [Dependency Mapping](#dependency-mapping)
- [Blockers and Risk Assessment](#blockers-and-risk-assessment)
- [Architecture Mapping](#architecture-mapping)
- [Phased Migration Plan](#phased-migration-plan)
- [Risks and Mitigations](#risks-and-mitigations)
- [Decision Log](#decision-log)

---

## Overview

### Why Migrate

- **Single binary, zero runtime** — Rust produces statically-linked binaries with no GC pauses and no runtime dependencies.
- **Eliminate CGO** — The Go build currently pulls in CGO via the `keyring` package (macOS Keychain, Linux libsecret). Rust's `keyring` crate uses native FFI without a C toolchain requirement, simplifying cross-compilation.
- **Smaller binaries** — Rust binaries are typically 2-5x smaller than equivalent Go binaries after stripping.
- **Memory safety without GC** — Ownership model eliminates an entire class of concurrency bugs at compile time.
- **Better WASM story** — Rust has first-class `wasm32` target support via `wasm-bindgen` and `wasm-pack`, compared to Go's `GOOS=js GOARCH=wasm` which requires shipping a `wasm_exec.js` runtime.

### What Pup Is

Pup is a CLI wrapper for the Datadog API. It provides OAuth2 + API key authentication across 43 API domain commands with 274+ subcommands. It supports JSON, YAML, and table output formatting, OS keychain token storage, and a WASM build target.

### Scope

| Metric | Value |
|--------|-------|
| Command files (cmd/) | 49 |
| Non-test Go source files | 76 |
| Test files | 65 |
| Non-test lines of code | ~20,000 |
| Total lines of code | ~38,000 |
| Registered commands | 47 (43 API domains + 4 utility) |
| Subcommands | 274+ |
| Direct dependencies | 8 |
| Unstable API operations | 63 |
| OAuth-excluded endpoint patterns | 52 |
| WASM-specific files | 3 |

---

## Dependency Mapping

All 8 direct Go dependencies have Rust equivalents.

| Go Dependency | Version | Rust Equivalent | Crate Version | Notes |
|--------------|---------|----------------|---------------|-------|
| `github.com/DataDog/datadog-api-client-go/v2` | v2.55.0 | `datadog-api-client-rust` | v0.27.0 | Pre-1.0, async-only (tokio). See [blockers](#blockers-and-risk-assessment). |
| `github.com/spf13/cobra` | v1.10.2 | `clap` | v4.x | Derive macros replace cobra's runtime registration. |
| `github.com/spf13/pflag` | v1.0.10 | `clap` (built-in) | — | Clap handles flags natively; no separate crate needed. |
| `github.com/99designs/keyring` | v1.2.2 | `keyring` | v3.x | Same name, same concept. macOS/Windows/Linux support. No CGO. |
| `github.com/google/uuid` | v1.6.0 | `uuid` | v1.x | Near-identical API. |
| `github.com/olekukonko/tablewriter` | v1.1.3 | `comfy-table` | v7.x | `tabled` is also an option. |
| `gopkg.in/yaml.v3` | v3.0.1 | `serde_yaml` | v0.9.x | Uses serde derive for (de)serialization. |
| `github.com/stretchr/testify` | v1.11.1 | — (built-in) | — | Rust's `assert!`, `assert_eq!`, `#[test]` + `#[should_panic]` cover this. |

### Additional Rust dependencies needed

| Crate | Purpose |
|-------|---------|
| `tokio` | Async runtime (required by datadog-api-client-rust) |
| `reqwest` | HTTP client (pulled in by DD client, also useful for OAuth flows) |
| `serde` + `serde_json` | JSON serialization (ubiquitous in Rust) |
| `oauth2` | OAuth2 PKCE flow (replaces hand-rolled `pkg/auth`) |
| `dirs` | XDG config directory resolution (replaces manual `~/.config/pup` logic) |
| `aes-gcm` | AES-256-GCM encryption for fallback token storage |
| `wasm-bindgen` | WASM target support (replaces `GOOS=js` build tags) |

---

## Blockers and Risk Assessment

### Critical — Must resolve before starting Phase 1

**None identified.** The Rust API client exists and covers all v1/v2 endpoints.

### Significant — Must resolve during Phase 1

#### 1. Rust API Client is Pre-1.0 (v0.27.0)

The `datadog-api-client-rust` crate is at v0.27.0 with no stable release. Breaking changes between minor versions are possible.

- **Impact:** API surface may shift during migration.
- **Mitigation:** Pin to a specific version. Vendor the crate if needed. The client is auto-generated from the same OpenAPI spec as the Go client, so feature parity is high.

#### 2. OAuth2 Bearer Token Injection

The Go client uses `datadog.ContextAccessToken` to inject OAuth bearer tokens into requests via Go's `context.Context`. The Rust client may not have an equivalent mechanism — it primarily supports API key auth via configuration.

- **Impact:** OAuth2 authentication (pup's primary auth flow) may require injecting `Authorization: Bearer <token>` headers manually.
- **Mitigation:** Investigate the Rust client's `configuration.bearer_access_token` field. If absent, wrap the HTTP client with middleware that injects the header. The Rust client uses `reqwest`, which supports middleware via `reqwest_middleware` or a custom `reqwest::Client` with default headers.

#### 3. Unstable Operation Naming Convention

The Go client uses `"v2.ListIncidents"` (PascalCase) for unstable operation IDs. The Rust client uses `"v2.list_incidents"` (snake_case). All 63 unstable operations need their IDs remapped.

- **Impact:** Low effort but easy to miss — a missed operation silently fails at runtime.
- **Mitigation:** Build a lookup table or write a codegen script. Verify each operation is enabled in integration tests.

### Moderate — Can be addressed during Phase 2-3

#### 4. OAuth-Excluded Endpoints (52 patterns across 7 API groups)

The Go codebase maintains a hand-curated list of endpoints that don't support OAuth (`pkg/client/auth_validator.go`). This list must be ported to Rust.

- **Affected API groups:** Logs (11), RUM (10), API/App Keys (8), Fleet (15), Notebooks (5), Error Tracking (2), Events (1)
- **Impact:** Porting the list is mechanical but must stay in sync with the Go version until cutover.
- **Mitigation:** Extract the list into a shared data format (JSON/YAML) that both Go and Rust read, or port it as a Rust module with tests.

#### 5. WASM Build Target

Go uses `//go:build js` build tags with 3 platform-specific files. Rust uses `#[cfg(target_arch = "wasm32")]` attributes and `wasm-bindgen` for JS interop, which is a fundamentally different approach.

- **Impact:** WASM auth flow (browser popup → redirect → token) needs to be reimplemented with `wasm-bindgen` and `web-sys`.
- **Mitigation:** Defer WASM to Phase 3. The WASM build is not on the critical path for CLI users.

#### 6. Volume — 274+ Subcommands

Each subcommand is a cobra `RunE` function that creates a client, calls an API method, and formats output. The pattern is mechanical but the sheer count is significant.

- **Impact:** This is the bulk of the migration effort.
- **Mitigation:** Most commands follow 2-3 patterns. Build code generators or macros to stamp them out. Prioritize by usage data.

### Non-Issues

#### CGO Elimination

The Go build requires CGO for `keyring` (macOS Keychain via `go-keychain`). The Rust `keyring` crate uses native FFI directly, eliminating the C toolchain dependency. This is a **net improvement**.

#### Cross-Compilation

Go's cross-compilation story is good but hampered by CGO. Rust's cross-compilation via `cross` or `cargo-zigbuild` is straightforward for all targets (macOS, Linux, Windows) without CGO complications.

---

## Architecture Mapping

### CLI Framework: `cobra` → `clap`

**Go (cobra — runtime registration):**
```go
var monitorsCmd = &cobra.Command{
    Use:   "monitors",
    Short: "Manage Datadog monitors",
}

func init() {
    rootCmd.AddCommand(monitorsCmd)
    monitorsCmd.AddCommand(monitorsListCmd)
}
```

**Rust (clap — derive macros, compile-time):**
```rust
#[derive(Parser)]
#[command(name = "pup")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage Datadog monitors
    Monitors(MonitorsArgs),
}
```

### Configuration: `viper` → `config` + `serde`

Go uses viper for config file + env var + flag merging. Rust has no single equivalent, but the pattern maps to:

| Go (viper) | Rust |
|-----------|------|
| Config file parsing | `serde_yaml` + custom `Config` struct |
| Env var binding | `std::env::var()` or `envy` crate |
| Flag binding | `clap` derive macros with `env` attribute |
| Precedence merging | Manual in `Config::load()` (flag > env > file > default) |

### Error Handling: `fmt.Errorf` → `thiserror` / `anyhow`

| Go | Rust |
|----|------|
| `fmt.Errorf("context: %w", err)` | `anyhow::Context` trait: `err.context("context")` |
| `errors.As(err, &target)` | `err.downcast_ref::<T>()` |
| Custom error types | `#[derive(thiserror::Error)]` enum |

### Context and Authentication: `context.Context` → Struct fields

Go threads authentication through `context.Context`:
```go
ctx = context.WithValue(ctx, datadog.ContextAccessToken, token)
```

Rust uses typed configuration:
```rust
let mut config = datadog::Configuration::new();
config.bearer_access_token = Some(token.to_string());
// or set API keys:
config.api_key = Some(ApiKey { key: api_key, prefix: None });
```

### Keychain Storage: `keyring` → `keyring`

Both use OS-native secure storage. The Rust crate has the same name and concept:

| Go | Rust |
|----|------|
| `keyring.Open(keyring.Config{...})` | `keyring::Entry::new("pup", "user")` |
| `ring.Set(keyring.Item{...})` | `entry.set_password("token")` |
| `ring.Get("key")` | `entry.get_password()` |

### Build Tags: `//go:build` → `#[cfg()]`

| Go | Rust |
|----|------|
| `//go:build js` | `#[cfg(target_arch = "wasm32")]` |
| `//go:build !js` | `#[cfg(not(target_arch = "wasm32"))]` |
| Separate files (`*_wasm.go`) | Same file with `cfg` attributes, or separate modules |

### Testing

| Go | Rust |
|----|------|
| `func TestFoo(t *testing.T)` | `#[test] fn test_foo()` |
| `assert.Equal(t, expected, actual)` | `assert_eq!(expected, actual)` |
| Table-driven tests | `#[test_case]` macro or loop in test fn |
| `testify/mock` | `mockall` crate |
| `t.Parallel()` | Tests run in parallel by default |
| Test binary (`go test`) | `cargo test` (built-in) |

---

## Phased Migration Plan

### Phase 0: Spike (1-2 weeks)

**Goal:** Prove the Rust API client works for pup's use cases. Validate OAuth token injection. Ship nothing.

**Deliverables:**
- [ ] Scaffold `pup-rs/` with `clap` CLI skeleton
- [ ] Wire up `datadog-api-client-rust` with API key auth
- [ ] Implement 3 representative commands:
  - `monitors list` — simple GET with query params
  - `logs search` — POST with body, OAuth-excluded (API key fallback)
  - `incidents list` — unstable operation
- [ ] Validate OAuth bearer token injection (find or build the mechanism)
- [ ] Validate unstable operation enabling with snake_case IDs
- [ ] Benchmark: binary size, startup time, memory usage vs Go

**Exit criteria:** All 3 commands produce identical output to the Go version. OAuth works. Unstable operations work.

### Phase 1: Foundation (3-4 weeks)

**Goal:** Port all shared infrastructure. No commands yet (except the 3 from Phase 0).

**Deliverables:**
- [ ] `pkg/config` → `src/config.rs` — Config loading with precedence (flag > env > file > default)
- [ ] `pkg/auth` → `src/auth/` — OAuth2 DCR + PKCE flow using `oauth2` crate
- [ ] `pkg/auth/storage` → `src/auth/storage.rs` — OS keychain via `keyring` crate + AES-256-GCM fallback
- [ ] `pkg/client` → `src/client.rs` — Datadog API client wrapper with:
  - OAuth bearer token injection
  - API key authentication
  - All 63 unstable operations enabled
  - OAuth-excluded endpoint fallback list (52 patterns)
- [ ] `pkg/formatter` → `src/formatter.rs` — JSON, YAML, table output + agent mode envelope
- [ ] `pkg/util` → `src/util.rs` — Time parsing, validation helpers
- [ ] `internal/version` → `src/version.rs` — Version and build info
- [ ] `pkg/agenthelp` → `src/agenthelp.rs` — Agent mode JSON schema generation
- [ ] `pkg/useragent` → `src/useragent.rs` — User-agent detection for agent mode
- [ ] CI pipeline: `cargo test`, `cargo clippy`, `cargo fmt --check`, coverage with `cargo-tarpaulin`
- [ ] Unit tests for all foundation modules (target: >80% coverage)

**Exit criteria:** `cargo test` passes. All foundation modules have unit tests. The 3 spike commands still work on top of the new foundation.

### Phase 2: Command Migration (6-8 weeks)

**Goal:** Port all 47 registered commands (274+ subcommands). Prioritize by usage.

#### Priority Tiers

**Tier 1 — High usage (port first):**

| Command | Subcommands | Notes |
|---------|-------------|-------|
| `auth` | login, logout, status, token | Critical path — must work before anything else |
| `monitors` | list, get, create, update, delete, mute, unmute, search, validate | Most-used domain |
| `metrics` | query, list, submit, metadata | Core observability |
| `logs` | search, tail, archives, custom-destinations, metrics | OAuth-excluded, needs API key fallback |
| `dashboards` | list, get, create, update, delete | Frequently used |
| `slos` | list, get, create, update, delete, history | Core SRE workflow |
| `synthetics` | list, get, create, update, delete, trigger, results | Complex subgroup structure |
| `events` | list, search | OAuth-excluded |

**Tier 2 — Medium usage:**

| Command | Notes |
|---------|-------|
| `incidents` | 16 unstable operations |
| `rum` | OAuth-excluded (10 patterns) |
| `apm` | Large command file (833 lines) |
| `traces` | Wrapper around logs |
| `on-call` | Multiple subgroups |
| `downtime` | Standard CRUD |
| `tags` | Standard CRUD |
| `infrastructure` | Hosts and containers |
| `users` | Standard CRUD |
| `cicd` | Pipelines and tests |

**Tier 3 — Lower usage / newer features:**

| Command | Notes |
|---------|-------|
| `cases` | 5 unstable operations |
| `security` | Rules, signals, suppressions |
| `integrations` | ServiceNow (9 unstable), Jira (7 unstable), OCI (2 unstable) |
| `api-keys` / `app-keys` | OAuth-excluded (8 patterns) |
| `audit-logs` | Standard search |
| `error-tracking` | OAuth-excluded |
| `notebooks` | V1 API, OAuth-excluded |
| `cloud` | AWS/Azure/GCP integrations |
| `cost` | Cloud cost management |
| `fleet` | 14 unstable + 15 OAuth-excluded |
| `status-pages` | Standard CRUD + third-party |
| `investigations` | Relatively new |
| `organizations` | Rarely used |
| `service-catalog` | Relatively new |
| `scorecards` | Relatively new |
| `usage` | Billing queries |
| `product-analytics` | Relatively new |
| `data-governance` | Relatively new |
| `obs-pipelines` | Relatively new |
| `network` | DNS, devices |
| `hamr` | 2 unstable operations |
| `code-coverage` | 2 unstable operations |
| `vulnerabilities` | Security vulnerabilities |
| `agent` | Agent management |

**Tier 4 — Utility commands:**

| Command | Notes |
|---------|-------|
| `alias` | Pure client-side, no API calls |
| `version` | Trivial |
| `test` | Config validation |
| `misc` | IP ranges, Graph snapshot |
| `static-analysis` | Placeholder (API endpoints pending) |

#### Migration pattern per command

Each command follows a mechanical translation:

1. Create `src/commands/<domain>.rs`
2. Define clap `Args` and `Subcommand` enums
3. Port each `RunE` function → async Rust function
4. Map Go API client calls to Rust client calls
5. Wire up output formatting
6. Write tests mirroring the Go test file

**Exit criteria:** All 274+ subcommands ported. `pup-rs <domain> <action> --help` matches Go version for every command. Integration smoke tests pass.

### Phase 3: Platform (2-3 weeks)

**Goal:** Build, release, and platform parity.

**Deliverables:**
- [ ] WASM build target (`wasm32-unknown-unknown` via `wasm-bindgen`)
  - Port `auth_wasm.go` → `auth_wasm.rs` with `#[cfg(target_arch = "wasm32")]`
  - Port `oauth_storage_wasm.go` → browser localStorage via `web-sys`
  - Port `keychain_wasm.go` → in-memory or localStorage fallback
- [ ] Cross-compilation matrix:
  - `x86_64-apple-darwin` / `aarch64-apple-darwin`
  - `x86_64-unknown-linux-gnu` / `aarch64-unknown-linux-gnu`
  - `x86_64-pc-windows-msvc`
  - `wasm32-unknown-unknown`
- [ ] CI/CD pipeline (GitHub Actions):
  - Test matrix across platforms
  - `cargo clippy` + `cargo fmt` checks
  - Coverage ≥ 80% enforcement
  - Binary size tracking
  - Release automation with `cargo-dist` or `cross`
- [ ] Homebrew formula update (`brew tap datadog-labs/pack`)
- [ ] Shell completions (`clap_complete` for bash, zsh, fish)

**Exit criteria:** Binaries build for all targets. CI is green. Homebrew install works.

### Phase 4: Parity and Cutover (2-3 weeks)

**Goal:** Validate full parity with Go version. Cut over.

**Deliverables:**
- [ ] Output parity validation — run both versions against Datadog sandbox, diff outputs
- [ ] Error message parity — same error format and status code handling
- [ ] Agent mode parity — JSON schema generation, agent envelope format
- [ ] Config file compatibility — reads existing `~/.config/pup/config.yaml`
- [ ] Keychain migration — reads tokens stored by Go version
- [ ] Performance benchmarks published (binary size, startup time, memory, throughput)
- [ ] Migration guide for users (if any CLI flags or behavior changed)
- [ ] Update all docs (README, CLAUDE.md, COMMANDS.md, etc.)
- [ ] Archive Go version, publish Rust version as `pup`

**Exit criteria:** Rust version passes all acceptance criteria. Go version archived. Users can `brew upgrade pup` seamlessly.

---

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Rust API client breaks in minor release | Medium | High | Pin version. Vendor if needed. Track upstream releases. |
| OAuth bearer token injection not supported by Rust client | Low | High | Phase 0 spike validates this. Fallback: reqwest middleware. |
| WASM build proves significantly harder in Rust | Medium | Low | WASM is not on critical path. Can defer or drop. |
| Unstable operations behave differently in Rust client | Low | Medium | Phase 0 spike validates. Integration tests catch regressions. |
| Migration takes longer than estimated | High | Medium | Phased approach allows shipping value incrementally. Go version remains operational throughout. |
| Rust API client missing endpoints present in Go client | Low | Medium | Both are generated from the same OpenAPI spec. Verify during Phase 0. |
| Team unfamiliarity with Rust slows development | Medium | Medium | Foundation (Phase 1) is the learning investment. Command migration (Phase 2) is mechanical. |
| Config/keychain format incompatibility between versions | Low | High | Use same config YAML format. Same keychain service/account names. Test migration explicitly. |

---

## Decision Log

Decisions to be made during migration. Record choices here as they are made.

| # | Decision | Status | Options | Chosen | Rationale |
|---|----------|--------|---------|--------|-----------|
| 1 | Async runtime | Pending | `tokio` vs `async-std` | — | DD Rust client requires tokio. Likely decided for us. |
| 2 | Error handling crate | Pending | `anyhow` vs `eyre` vs `thiserror` only | — | `anyhow` for app-level, `thiserror` for library types is conventional. |
| 3 | Table output crate | Pending | `comfy-table` vs `tabled` vs `prettytable-rs` | — | Evaluate feature parity with current tablewriter output. |
| 4 | WASM bundler | Pending | `wasm-pack` vs `trunk` vs manual `wasm-bindgen` | — | Defer to Phase 3. |
| 5 | Monorepo or separate repo | Pending | Same repo (`pup-rs/`) vs new repo | — | Same repo simplifies sharing test fixtures and docs. |
| 6 | Release tooling | Pending | `cargo-dist` vs `cross` + manual | — | Evaluate during Phase 3. |
| 7 | Code generation for commands | Pending | Rust macros vs external codegen vs manual | — | Evaluate after Phase 0 shows the pattern. |
| 8 | Minimum Rust version (MSRV) | Pending | Latest stable vs pinned MSRV | — | Pinned MSRV aids distro packaging. |

---

## Appendix: Command Group Inventory

All 47 registered commands from `cmd/root.go`:

| # | Command | File | Lines | Unstable Ops | OAuth-Excluded | Notes |
|---|---------|------|-------|-------------|----------------|-------|
| 1 | `agent` | agent.go | 114 | — | — | |
| 2 | `alias` | alias.go | 299 | — | — | Client-side only |
| 3 | `api-keys` | api_keys.go | 209 | — | 4 patterns | |
| 4 | `app-keys` | app_keys.go | 205 | — | 4 patterns | |
| 5 | `apm` | apm.go | 833 | — | — | Large file |
| 6 | `audit-logs` | audit_logs.go | 138 | — | — | |
| 7 | `auth` | auth.go | 555 | — | — | + auth_wasm.go (71 lines) |
| 8 | `cases` | cases.go | 973 | 5 | — | |
| 9 | `cicd` | cicd.go | 696 | — | — | |
| 10 | `cloud` | cloud.go | 335 | — | — | |
| 11 | `code-coverage` | code_coverage.go | 100 | 2 | — | |
| 12 | `cost` | cost.go | 244 | — | — | |
| 13 | `dashboards` | dashboards.go | 273 | — | — | |
| 14 | `data-governance` | data_governance.go | 75 | — | — | |
| 15 | `downtime` | downtime.go | 138 | — | — | |
| 16 | `error-tracking` | error_tracking.go | 196 | — | 2 patterns | |
| 17 | `events` | events.go | 188 | — | 1 pattern | |
| 18 | `fleet` | fleet.go | 517 | 14 | 15 patterns | |
| 19 | `hamr` | hamr.go | 103 | 2 | — | |
| 20 | `incidents` | incidents.go | 726 | 16 | — | |
| 21 | `infrastructure` | infrastructure.go | 120 | — | — | |
| 22 | `integrations` | integrations.go | 683 | 18 | — | ServiceNow (9), Jira (7), OCI (2) |
| 23 | `investigations` | investigations.go | 232 | — | — | |
| 24 | `logs` | logs_simple.go | 1322 | — | 11 patterns | Largest file |
| 25 | `metrics` | metrics.go | 941 | — | — | |
| 26 | `misc` | miscellaneous.go | 78 | — | — | IP ranges, graph snapshot |
| 27 | `monitors` | monitors.go | 434 | — | — | |
| 28 | `network` | network.go | 86 | — | — | |
| 29 | `notebooks` | notebooks.go | 248 | — | 5 patterns | V1 API |
| 30 | `obs-pipelines` | obs_pipelines.go | 79 | — | — | |
| 31 | `on-call` | on_call.go | 566 | — | — | |
| 32 | `organizations` | organizations.go | 87 | — | — | |
| 33 | `product-analytics` | product_analytics.go | 149 | — | — | |
| 34 | `rum` | rum.go | 779 | — | 10 patterns | |
| 35 | `scorecards` | scorecards.go | 79 | — | — | |
| 36 | `security` | security.go | 427 | — | — | |
| 37 | `service-catalog` | service_catalog.go | 92 | — | — | |
| 38 | `slos` | slos.go | 357 | 1 | — | SLO status (unstable) |
| 39 | `static-analysis` | *(via cicd.go)* | — | — | — | Placeholder |
| 40 | `status-pages` | status_pages.go | 630 | — | — | + third_party (251 lines) |
| 41 | `synthetics` | synthetics.go | 432 | — | — | |
| 42 | `tags` | tags.go | 202 | — | — | |
| 43 | `test` | root.go | — | — | — | Config validation |
| 44 | `traces` | traces_simple.go | 21 | — | — | Wrapper around logs |
| 45 | `usage` | usage.go | 123 | — | — | |
| 46 | `users` | users.go | 123 | — | — | |
| 47 | `version` | root.go | — | — | — | Trivial |

**Totals:** 63 unstable operations, 52 OAuth-excluded endpoint patterns across 7 API groups.

---

## Appendix: Unstable Operations (63)

All operations enabled via `configuration.SetUnstableOperationEnabled()` in `pkg/client/client.go`:

| Category | Count | Operations |
|----------|-------|------------|
| Incidents | 16 | ListIncidents, GetIncident, CreateIncident, UpdateIncident, DeleteIncident, CreateGlobalIncidentHandle, DeleteGlobalIncidentHandle, GetGlobalIncidentSettings, ListGlobalIncidentHandles, UpdateGlobalIncidentHandle, UpdateGlobalIncidentSettings, Create/Delete/Get/List/UpdateIncidentPostmortemTemplate |
| Fleet Automation | 14 | ListFleetAgents, GetFleetAgentInfo, ListFleetAgentVersions, ListFleetDeployments, GetFleetDeployment, CreateFleetDeploymentConfigure, CreateFleetDeploymentUpgrade, CancelFleetDeployment, ListFleetSchedules, GetFleetSchedule, Create/Update/Delete/TriggerFleetSchedule |
| ServiceNow | 9 | Create/Delete/Get/UpdateServiceNowTemplate, ListServiceNowAssignmentGroups, ListServiceNowBusinessServices, ListServiceNowInstances, ListServiceNowTemplates, ListServiceNowUsers |
| Jira | 7 | Create/Delete/Get/UpdateJiraIssueTemplate, DeleteJiraAccount, ListJiraAccounts, ListJiraIssueTemplates |
| Cases | 5 | CreateCaseJiraIssue, LinkJiraIssueToCase, UnlinkJiraIssue, CreateCaseServiceNowTicket, MoveCaseToProject |
| Content Packs | 3 | ActivateContentPack, DeactivateContentPack, GetContentPacksStates |
| Code Coverage | 2 | GetCodeCoverageBranchSummary, GetCodeCoverageCommitSummary |
| OCI Integration | 2 | CreateTenancyConfig, GetTenancyConfigs |
| HAMR | 2 | CreateHamrOrgConnection, GetHamrOrgConnection |
| Entity Risk Scores | 1 | ListEntityRiskScores |
| SLO Status | 1 | GetSloStatus |
| Flaky Tests | 1 | UpdateFlakyTests |
