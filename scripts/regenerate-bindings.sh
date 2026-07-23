#!/usr/bin/env bash
# Regenerate pre-built bindings for freerdp-sys.
#
# Prerequisites:
#   - FreeRDP submodule initialized: git submodule update --init --recursive
#   - libclang installed (for bindgen)
#   - CMake >= 3.13
#   - C compiler + OpenSSL + zlib dev headers
#
# Usage:
#   ./scripts/regenerate-bindings.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_ROOT"

echo "==> Building with bindgen to generate fresh bindings..."
cargo build --features vendored,generate-bindings 2>&1

echo "==> Locating generated bindings..."
BINDINGS_FILE=$(find target -path "*/freerdp-sys-*/out/bindings.rs" -type f | head -1)

if [[ -z "$BINDINGS_FILE" ]]; then
    echo "ERROR: Could not find generated bindings.rs in target/"
    exit 1
fi

echo "==> Copying bindings to src/bindings.rs"
cp "$BINDINGS_FILE" src/bindings.rs

echo "==> Formatting bindings..."
rustfmt src/bindings.rs 2>/dev/null || true

echo "==> Done! src/bindings.rs has been updated."
echo "    Review the diff and commit if satisfied."
