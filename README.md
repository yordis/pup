# Fetch - Datadog API CLI Wrapper

A Go-based command-line wrapper for easy interaction with Datadog APIs.

## Features

- **Native Go Implementation**: Fast, cross-platform binary
- **Simple Commands**: Intuitive CLI for common Datadog operations
- **Multiple Authentication**: Support for API key authentication (OAuth2 coming soon)
- **JSON Output**: Structured output for easy parsing and automation

## Installation

```bash
# Clone the repository
git clone https://github.com/DataDog/fetch.git
cd fetch

# Build
go build -o fetch .

# Install (optional)
go install
```

## Configuration

Set your Datadog credentials as environment variables:

```bash
export DD_API_KEY="your-datadog-api-key"
export DD_APP_KEY="your-datadog-application-key"
export DD_SITE="datadoghq.com"  # Optional, defaults to datadoghq.com
```

## Usage

### Test Connection

```bash
fetch test
```

### Monitors

```bash
# List all monitors
fetch monitors list

# Get specific monitor
fetch monitors get 12345678

# Delete monitor
fetch monitors delete 12345678 --yes
```

### Dashboards

```bash
# List all dashboards
fetch dashboards list

# Get dashboard details
fetch dashboards get abc-123-def

# Delete dashboard
fetch dashboards delete abc-123-def --yes
```

### SLOs

```bash
# List all SLOs
fetch slos list

# Get SLO details
fetch slos get abc-123

# Delete SLO
fetch slos delete abc-123 --yes
```

### Incidents

```bash
# List all incidents
fetch incidents list

# Get incident details
fetch incidents get abc-123-def
```

## Global Flags

- `-o, --output`: Output format (json, table, yaml) - default: json
- `-y, --yes`: Skip confirmation prompts for destructive operations

## Environment Variables

- `DD_API_KEY`: Datadog API key (required)
- `DD_APP_KEY`: Datadog Application key (required)
- `DD_SITE`: Datadog site (default: datadoghq.com)
- `DD_AUTO_APPROVE`: Auto-approve destructive operations (true/false)

## Development

```bash
# Run tests
go test ./...

# Build
go build -o fetch .

# Run without building
go run main.go monitors list
```

## License

Apache License 2.0 - see LICENSE for details.

## Documentation

For detailed documentation, see [CLAUDE.md](CLAUDE.md).
