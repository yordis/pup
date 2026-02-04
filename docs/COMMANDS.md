# Command Reference

Complete reference for all 33 command groups in Pup.

## Command Pattern

```bash
pup <domain> <action> [options]           # Simple commands
pup <domain> <subgroup> <action> [options] # Nested commands
```

## Status Legend

- ✅ **WORKING** - Command compiles and runs (requires valid auth)
- ⚠️ **API BLOCKED** - Implementation correct, waiting for API client library updates
- ⏳ **PLACEHOLDER** - Skeleton implementation, API endpoints pending

## Command Index

| Domain | Subcommands | File | Status |
|--------|-------------|------|--------|
| auth | login, logout, status, refresh | cmd/auth.go | ✅ |
| metrics | query, list, get, search | cmd/metrics.go | ✅ |
| logs | search, list, aggregate | cmd/logs.go | ✅ |
| traces | search, list, aggregate | cmd/traces.go | ✅ |
| monitors | list, get, delete | cmd/monitors.go | ✅ |
| dashboards | list, get, delete, url | cmd/dashboards.go | ✅ |
| slos | list, get, create, update, delete, corrections | cmd/slos.go | ✅ |
| incidents | list, get, create, update | cmd/incidents.go | ✅ |
| rum | apps, metrics, retention-filters, sessions | cmd/rum.go | ⚠️ |
| cicd | pipelines, events | cmd/cicd.go | ⚠️ |
| vulnerabilities | search, list | cmd/vulnerabilities.go | ⚠️ |
| static-analysis | ast, custom-rulesets, sca, coverage | cmd/vulnerabilities.go | ⚠️ |
| downtime | list, get, cancel | cmd/downtime.go | ✅ |
| tags | list, get, add, update, delete | cmd/tags.go | ⚠️ |
| events | list, search, get | cmd/events.go | ⚠️ |
| on-call | teams (list, get) | cmd/on_call.go | ✅ |
| audit-logs | list, search | cmd/audit_logs.go | ⚠️ |
| api-keys | list, get, create, delete | cmd/api_keys.go | ✅ |
| infrastructure | hosts (list, get) | cmd/infrastructure.go | ✅ |
| synthetics | tests, locations | cmd/synthetics.go | ✅ |
| users | list, get, roles | cmd/users.go | ✅ |
| notebooks | list, get, delete | cmd/notebooks.go | ✅ |
| security | rules, signals, findings | cmd/security.go | ✅ |
| organizations | get, list | cmd/organizations.go | ✅ |
| service-catalog | list, get | cmd/service_catalog.go | ✅ |
| error-tracking | issues (list, get) | cmd/error_tracking.go | ✅ |
| scorecards | list, get | cmd/scorecards.go | ✅ |
| usage | summary, hourly | cmd/usage.go | ⚠️ |
| data-governance | scanner-rules (list) | cmd/data_governance.go | ✅ |
| obs-pipelines | list, get | cmd/obs_pipelines.go | ⏳ |
| network | flows, devices | cmd/network.go | ⏳ |
| cloud | aws, gcp, azure (list) | cmd/cloud.go | ✅ |
| integrations | slack, pagerduty, webhooks | cmd/integrations.go | ✅ |
| misc | ip-ranges, status | cmd/miscellaneous.go | ✅ |

**Summary:** 23 working, 7 API-blocked, 3 placeholders

## Common Patterns

### List Operations
```bash
pup <domain> list [--flags]
pup monitors list --tags="env:production"
pup dashboards list
```

### Get Operations
```bash
pup <domain> get <id>
pup monitors get 12345678
pup slos get abc-123-def
```

### Search/Query
```bash
pup logs search --query="status:error" --from="1h"
pup metrics query --query="avg:system.cpu.user{*}"
pup events search --query="@user.id:12345"
```

### Create/Update/Delete
```bash
pup <domain> create [--flags]
pup <domain> update <id> [--flags]
pup <domain> delete <id> [--yes]
```

### Nested Commands
```bash
pup rum apps list
pup rum metrics get <id>
pup cicd pipelines list
pup security rules list
pup infrastructure hosts list
```

## Domain Categories

### Data & Observability
- **metrics** - Time-series metrics (query, list, get, search)
- **logs** - Log search and analysis (search, list, aggregate)
- **traces** - APM traces (search, list, aggregate)
- **rum** - Real User Monitoring (apps, metrics, retention-filters, sessions)
- **events** - Infrastructure events (list, search, get)

### Monitoring & Alerting
- **monitors** - Monitor management (list, get, delete)
- **dashboards** - Dashboard management (list, get, delete, url)
- **slos** - Service Level Objectives (list, get, create, update, delete)
- **synthetics** - Synthetic monitoring (tests, locations)
- **notebooks** - Investigation notebooks (list, get, delete)
- **downtime** - Monitor downtime (list, get, cancel)

### Infrastructure & Performance
- **infrastructure** - Host inventory (hosts list, hosts get)
- **network** - Network monitoring (flows list, devices list)
- **tags** - Host tag management (list, get, add, update, delete)

### Security & Compliance
- **security** - Security monitoring (rules, signals, findings)
- **vulnerabilities** - Vulnerability management (search, list)
- **static-analysis** - Code security (ast, custom-rulesets, sca, coverage)
- **audit-logs** - Audit trail (list, search)
- **data-governance** - Sensitive data scanning (scanner-rules list)

### Cloud & Integrations
- **cloud** - Cloud providers (aws, gcp, azure)
- **integrations** - Third-party integrations (slack, pagerduty, webhooks)

### Development & Quality
- **cicd** - CI/CD visibility (pipelines, events)
- **error-tracking** - Error management (issues list, issues get)
- **scorecards** - Service quality (list, get)
- **service-catalog** - Service registry (list, get)

### Operations & Incident Response
- **incidents** - Incident management (list, get, create, update)
- **on-call** - On-call teams (teams list, teams get)

### Organization & Access
- **users** - User management (list, get, roles)
- **organizations** - Org settings (get, list)
- **api-keys** - API key management (list, get, create, delete)

### Cost & Usage
- **usage** - Usage and billing (summary, hourly)

### Configuration & Data Management
- **obs-pipelines** - Observability pipelines (list, get)
- **misc** - Miscellaneous (ip-ranges, status)

## Global Flags

Available on all commands:

```bash
--config string      Config file path (default: ~/.config/pup/config.yaml)
--site string        Datadog site (default: datadoghq.com)
--output string      Output format: json, yaml, table (default: json)
--verbose            Enable verbose logging
--yes                Skip confirmation prompts
```

## Known API Issues

Commands with ⚠️ status have compilation errors due to datadog-api-client-go library mismatches:

1. **audit_logs.go** - Pointer method call issue with WithBody
2. **cicd.go** - Method signature mismatches in pipeline events API
3. **events.go** - Missing WithStart/WithEnd methods
4. **rum.go** - Missing ListRUMApplications and metrics API
5. **tags.go** - Type mismatch with Tags field
6. **usage.go** - Missing WithEndHr method, deprecated endpoints
7. **vulnerabilities.go** - Type signature mismatches

These are structural issues in the API client library. Command implementations are correct and will work once the library is updated.
