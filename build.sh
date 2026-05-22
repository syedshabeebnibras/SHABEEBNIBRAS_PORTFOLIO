#!/usr/bin/env bash
# Vercel build script — bootstraps Rust (if missing) + Trunk, plants a
# wasm-opt pass-through shim, then builds the release WASM bundle.

set -euo pipefail

TRUNK_VERSION=0.21.5

# ─── 0) source any existing rust env BEFORE checking command -v ─────────
# /rust/env       → vercel's build image (rust pre-installed but not on PATH)
# $HOME/.cargo/env → standard rustup install
[ -f /rust/env ] && . /rust/env
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

# ─── 1) Rust toolchain ──────────────────────────────────────────────────
if command -v rustup >/dev/null 2>&1; then
  echo "[1/5] rust already installed: $(rustup --version 2>/dev/null || true)"
else
  echo "[1/5] installing rustup (silent)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
    sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path
  . "$HOME/.cargo/env"
fi

# ─── 2) wasm32 target ───────────────────────────────────────────────────
echo "[2/5] adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# ─── 3) Trunk (prebuilt, musl-static for portability) ───────────────────
echo "[3/5] installing trunk ${TRUNK_VERSION} (prebuilt, musl-static)..."
mkdir -p ./bin
curl -sSL \
  "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-musl.tar.gz" |
  tar -xz -C ./bin
chmod +x ./bin/trunk
export PATH="$PWD/bin:$PATH"
trunk --version

# ─── 4) wasm-opt pass-through shim ──────────────────────────────────────
# Trunk's bundled Binaryen 116 rejects bulk-memory ops (memory.copy,
# memory.fill) that rustc emits by default in modern Rust, and Trunk
# doesn't expose wasm-opt --enable-bulk-memory. We plant a shim at the
# exact path Trunk caches its wasm-opt binary so it skips downloading
# and uses ours instead — which just copies input to output.
#
# Trade-off: skips wasm-opt's size optimization. Bundle is ~30-50% larger.
echo "[4/5] installing wasm-opt pass-through shim..."
WASM_OPT_DIR="${HOME:-/vercel}/.cache/trunk/wasm-opt-version_116/bin"
mkdir -p "$WASM_OPT_DIR"
cat > "$WASM_OPT_DIR/wasm-opt" <<'SHIM'
#!/usr/bin/env bash
# wasm-opt pass-through shim: parses Trunk's CLI invocation
# (`--output=PATH [-O...] INPUT`) and just copies the input to the output.
output=""
input=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --output=*) output="${1#--output=}"; shift ;;
    --output)   output="$2"; shift 2 ;;
    -O*)        shift ;;     # ignore optimization level
    -*)         shift ;;     # ignore any other flag
    *)          input="$1"; shift ;;
  esac
done
mkdir -p "$(dirname "$output")"
cp "$input" "$output"
SHIM
chmod +x "$WASM_OPT_DIR/wasm-opt"
echo "  → $WASM_OPT_DIR/wasm-opt (shim installed)"

# ─── 5) build ───────────────────────────────────────────────────────────
echo "[5/5] building release wasm bundle..."
trunk build --release

echo "[done] dist/ ready for vercel"
ls -la dist/
