#!/usr/bin/env bash
#
# run_comparison.sh -- End-to-end comparison test harness for Go pup vs Rust pup-rs.
#
# 1. Starts the mock Datadog API server.
# 2. Runs every command from go_commands.txt against the Go binary.
# 3. Saves the Go request log.
# 4. Clears the server log.
# 5. Runs every command from rust_commands.txt against the Rust binary.
# 6. Saves the Rust request log.
# 7. Kills the mock server.
# 8. Runs the comparison script.
#
# Environment variables honoured:
#   MOCK_PORT     -- port for the mock server (default: 19876)
#   GO_BINARY     -- path to the Go pup binary (default: ../../pup)
#   RUST_BINARY   -- path to the Rust pup-rs binary (default: target/release/pup-rs)
#   KEEP_LOGS     -- if non-empty, keep intermediate log files
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
MOCK_DIR="$SCRIPT_DIR/mock_server"

MOCK_PORT="${MOCK_PORT:-19876}"
GO_BINARY="${GO_BINARY:-$PROJECT_ROOT/../pup}"
RUST_BINARY="${RUST_BINARY:-$PROJECT_ROOT/target/release/pup-rs}"
REQUEST_LOG="/tmp/pup_mock_requests.jsonl"
GO_LOG="/tmp/pup_comparison_go.jsonl"
RUST_LOG="/tmp/pup_comparison_rust.jsonl"
GO_CMDS="$SCRIPT_DIR/go_commands.txt"
RUST_CMDS="$SCRIPT_DIR/rust_commands.txt"

# -- Colours for output ---------------------------------------------------
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No colour

info()  { printf "${CYAN}[INFO]${NC}  %s\n" "$*"; }
warn()  { printf "${YELLOW}[WARN]${NC}  %s\n" "$*"; }
err()   { printf "${RED}[ERR]${NC}   %s\n" "$*" >&2; }
ok()    { printf "${GREEN}[OK]${NC}    %s\n" "$*"; }

# -- Preflight checks -----------------------------------------------------
check_file() {
    if [[ ! -f "$1" ]]; then
        err "Required file not found: $1"
        exit 1
    fi
}

check_file "$MOCK_DIR/server.py"
check_file "$SCRIPT_DIR/compare_requests.py"

if [[ ! -f "$GO_CMDS" ]]; then
    warn "go_commands.txt not found -- run generate_commands.sh first"
    warn "Creating empty file so the harness can proceed"
    touch "$GO_CMDS"
fi
if [[ ! -f "$RUST_CMDS" ]]; then
    warn "rust_commands.txt not found -- run generate_commands.sh first"
    warn "Creating empty file so the harness can proceed"
    touch "$RUST_CMDS"
fi

if [[ ! -x "$GO_BINARY" ]] && [[ -f "$GO_BINARY" ]]; then
    warn "Go binary exists but is not executable: $GO_BINARY"
fi
if [[ ! -x "$RUST_BINARY" ]] && [[ -f "$RUST_BINARY" ]]; then
    warn "Rust binary exists but is not executable: $RUST_BINARY"
fi

# -- Cleanup trap ----------------------------------------------------------
MOCK_PID=""
cleanup() {
    if [[ -n "$MOCK_PID" ]]; then
        info "Stopping mock server (PID $MOCK_PID)..."
        kill "$MOCK_PID" 2>/dev/null || true
        wait "$MOCK_PID" 2>/dev/null || true
    fi
    if [[ -z "${KEEP_LOGS:-}" ]]; then
        rm -f "$REQUEST_LOG"
    fi
}
trap cleanup EXIT

# -- Start mock server -----------------------------------------------------
info "Starting mock Datadog API server on port $MOCK_PORT..."
python3 "$MOCK_DIR/server.py" "$MOCK_PORT" &
MOCK_PID=$!

# Wait for the server to be ready (up to 5 seconds).
for i in $(seq 1 50); do
    if curl -sf "http://127.0.0.1:$MOCK_PORT/api/v1/validate" >/dev/null 2>&1; then
        break
    fi
    if ! kill -0 "$MOCK_PID" 2>/dev/null; then
        err "Mock server exited prematurely"
        exit 1
    fi
    sleep 0.1
done

if ! curl -sf "http://127.0.0.1:$MOCK_PORT/api/v1/validate" >/dev/null 2>&1; then
    err "Mock server did not become ready in time"
    exit 1
fi
ok "Mock server ready (PID $MOCK_PID)"

# -- Common environment for both CLIs --------------------------------------
export DD_API_KEY="test-key"
export DD_APP_KEY="test-app-key"
export DD_SITE="127.0.0.1"
export PUP_MOCK_SERVER="http://127.0.0.1:$MOCK_PORT"

# Some CLIs may need the base URL set differently; export both forms.
export DD_API_URL="http://127.0.0.1:$MOCK_PORT"
export DATADOG_HOST="http://127.0.0.1:$MOCK_PORT"

# -- Helper: run a commands file through a binary --------------------------
run_commands() {
    local binary="$1"
    local cmds_file="$2"
    local label="$3"
    local count=0
    local failed=0

    if [[ ! -f "$binary" ]]; then
        warn "Binary not found ($label): $binary -- skipping"
        return 0
    fi

    info "Running $label commands from $(basename "$cmds_file")..."
    while IFS= read -r line || [[ -n "$line" ]]; do
        # Skip blank lines and comments.
        [[ -z "$line" || "$line" == \#* ]] && continue

        count=$((count + 1))
        # Replace the placeholder binary name with the actual path.
        # Command lines are expected to start with the binary name (e.g. "pup ..."
        # or "pup-rs ...") which we strip and replace.
        local args
        args="${line#* }"  # everything after the first space
        if ! timeout 10 "$binary" $args >/dev/null 2>&1; then
            failed=$((failed + 1))
        fi
    done < "$cmds_file"

    if [[ $count -eq 0 ]]; then
        warn "No commands found in $cmds_file"
    else
        info "$label: ran $count commands ($failed failed)"
    fi
}

# -- Phase 1: Go binary ---------------------------------------------------
info "===== Phase 1: Go binary ====="
> "$REQUEST_LOG"  # truncate
run_commands "$GO_BINARY" "$GO_CMDS" "Go"
cp "$REQUEST_LOG" "$GO_LOG"
ok "Go request log saved to $GO_LOG ($(wc -l < "$GO_LOG" | tr -d ' ') requests)"

# -- Phase 2: Rust binary -------------------------------------------------
info "===== Phase 2: Rust binary ====="
> "$REQUEST_LOG"  # truncate
run_commands "$RUST_BINARY" "$RUST_CMDS" "Rust"
cp "$REQUEST_LOG" "$RUST_LOG"
ok "Rust request log saved to $RUST_LOG ($(wc -l < "$RUST_LOG" | tr -d ' ') requests)"

# -- Phase 3: Stop mock server --------------------------------------------
info "Stopping mock server..."
kill "$MOCK_PID" 2>/dev/null || true
wait "$MOCK_PID" 2>/dev/null || true
MOCK_PID=""
ok "Mock server stopped"

# -- Phase 4: Compare -----------------------------------------------------
info "===== Phase 3: Comparison ====="
python3 "$SCRIPT_DIR/compare_requests.py" "$GO_LOG" "$RUST_LOG"
RESULT=$?

echo ""
if [[ $RESULT -eq 0 ]]; then
    ok "Full API parity achieved!"
else
    warn "Gaps remain -- see comparison output above"
fi

exit $RESULT
