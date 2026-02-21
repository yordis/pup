#!/usr/bin/env bash
#
# generate_commands.sh -- Generate go_commands.txt and rust_commands.txt by
# walking the help output of both CLIs recursively and producing test
# invocations for every leaf subcommand.
#
# Usage:
#     ./generate_commands.sh [--go-binary PATH] [--rust-binary PATH]
#
# Environment variables:
#   GO_BINARY   -- path to the Go pup binary (default: ../../pup)
#   RUST_BINARY -- path to the Rust pup-rs binary (default: target/release/pup-rs)
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

GO_BINARY="${GO_BINARY:-$PROJECT_ROOT/../pup}"
RUST_BINARY="${RUST_BINARY:-$PROJECT_ROOT/target/release/pup-rs}"
GO_CMDS="$SCRIPT_DIR/go_commands.txt"
RUST_CMDS="$SCRIPT_DIR/rust_commands.txt"
FIXTURE_DIR="$SCRIPT_DIR/mock_server/fixtures"

# Parse CLI args
while [[ $# -gt 0 ]]; do
    case "$1" in
        --go-binary)   GO_BINARY="$2"; shift 2;;
        --rust-binary) RUST_BINARY="$2"; shift 2;;
        *)             echo "Unknown arg: $1" >&2; exit 1;;
    esac
done

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Classify a subcommand name and return test arguments.
# We use simple heuristics based on the command name.
classify_and_args() {
    local cmd_name="$1"

    case "$cmd_name" in
        list|search|ls|index|find|query|get-all)
            # List/search commands: no extra args needed
            echo ""
            ;;
        get|show|inspect|view|describe)
            # Get by ID
            echo "test-id-123"
            ;;
        create|new|add)
            # Create: provide fixture file
            echo "--file $FIXTURE_DIR/v2_ok.json"
            ;;
        update|edit|modify|patch|put)
            # Update: ID + fixture
            echo "test-id-123 --file $FIXTURE_DIR/v2_ok.json"
            ;;
        delete|remove|destroy|rm)
            # Delete by ID
            echo "test-id-123"
            ;;
        mute|unmute|pause|cancel|resolve|enable|disable|trigger|start|stop)
            # Action on an ID
            echo "test-id-123"
            ;;
        aggregate|analytics)
            # Aggregation queries
            echo ""
            ;;
        validate|check|test|ping|verify)
            # Validation commands
            echo ""
            ;;
        *)
            # Default: try running with no args; many commands are list-like
            echo ""
            ;;
    esac
}

# Extract leaf subcommands from a CLI binary by walking help output.
# Outputs lines like: "domain subcommand" or "domain subgroup subcommand"
extract_commands() {
    local binary="$1"
    local binary_name
    binary_name="$(basename "$binary")"

    if [[ ! -x "$binary" ]]; then
        echo "WARNING: binary not found or not executable: $binary" >&2
        return
    fi

    # Get top-level commands (skip help, completion, version, auth).
    local top_cmds
    top_cmds=$("$binary" --help 2>/dev/null | \
        awk '/^Available Commands:/,/^$/' | \
        grep -E '^\s+\w' | \
        awk '{print $1}' | \
        grep -vE '^(help|completion|version|auth|__complete)$' || true)

    for domain in $top_cmds; do
        # Get subcommands for this domain.
        local sub_cmds
        sub_cmds=$("$binary" "$domain" --help 2>/dev/null | \
            awk '/^Available Commands:/,/^$/' | \
            grep -E '^\s+\w' | \
            awk '{print $1}' | \
            grep -vE '^(help)$' || true)

        if [[ -z "$sub_cmds" ]]; then
            # This domain is itself a leaf command.
            echo "$domain"
            continue
        fi

        for sub in $sub_cmds; do
            # Check if this subcommand has further subcommands (3-level nesting).
            local sub_sub_cmds
            sub_sub_cmds=$("$binary" "$domain" "$sub" --help 2>/dev/null | \
                awk '/^Available Commands:/,/^$/' | \
                grep -E '^\s+\w' | \
                awk '{print $1}' | \
                grep -vE '^(help)$' || true)

            if [[ -z "$sub_sub_cmds" ]]; then
                # Two-level: domain subcommand
                echo "$domain $sub"
            else
                # Three-level: domain subgroup subcommand
                for subsub in $sub_sub_cmds; do
                    echo "$domain $sub $subsub"
                done
            fi
        done
    done
}

# Generate a commands file from extracted leaf commands.
generate_commands_file() {
    local binary="$1"
    local output_file="$2"
    local binary_name
    binary_name="$(basename "$binary")"

    echo "# Auto-generated test commands for $binary_name" > "$output_file"
    echo "# Generated: $(date -u '+%Y-%m-%dT%H:%M:%SZ')" >> "$output_file"
    echo "#" >> "$output_file"
    echo "# Each line is a full CLI invocation. The harness will replace" >> "$output_file"
    echo "# the binary name with the actual path at runtime." >> "$output_file"
    echo "" >> "$output_file"

    local count=0

    while IFS= read -r cmd_path; do
        [[ -z "$cmd_path" ]] && continue

        # The last word in the command path is the "action".
        local action
        action="${cmd_path##* }"

        local extra_args
        extra_args="$(classify_and_args "$action")"

        # Build the full command line.
        local full_cmd="$binary_name $cmd_path"
        if [[ -n "$extra_args" ]]; then
            full_cmd="$full_cmd $extra_args"
        fi

        echo "$full_cmd" >> "$output_file"
        count=$((count + 1))
    done < <(extract_commands "$binary")

    echo "" >> "$output_file"
    echo "# Total: $count commands" >> "$output_file"
    echo "Generated $count commands -> $output_file"
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

echo "=== Command Generator ==="
echo "Go binary:   $GO_BINARY"
echo "Rust binary: $RUST_BINARY"
echo ""

if [[ -x "$GO_BINARY" ]]; then
    echo "--- Generating Go commands ---"
    generate_commands_file "$GO_BINARY" "$GO_CMDS"
else
    echo "WARNING: Go binary not found at $GO_BINARY -- skipping"
    echo "# Go binary not found -- no commands generated" > "$GO_CMDS"
fi

echo ""

if [[ -x "$RUST_BINARY" ]]; then
    echo "--- Generating Rust commands ---"
    generate_commands_file "$RUST_BINARY" "$RUST_CMDS"
else
    echo "WARNING: Rust binary not found at $RUST_BINARY -- skipping"
    echo "# Rust binary not found -- no commands generated" > "$RUST_CMDS"
fi

echo ""
echo "Done. Command files:"
echo "  Go:   $GO_CMDS"
echo "  Rust: $RUST_CMDS"
echo ""
echo "Next: run ./run_comparison.sh to execute the comparison test."
