#!/usr/bin/env bash
# Vercel build script — bootstraps Rust toolchain (if missing) and Trunk,
# then builds the release WASM bundle.
#
# Vercel's build image ships Rust pre-installed at /rust/bin but it is NOT
# on PATH by default. We have to source /rust/env first to expose rustup,
# THEN decide whether an install is needed.

set -euo pipefail

TRUNK_VERSION=0.21.5

# ─── 0) source any existing rust env BEFORE checking command -v ─────────
# /rust/env       → vercel's build image
# $HOME/.cargo/env → standard rustup install
[ -f /rust/env ] && . /rust/env
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

# ─── 1) Rust toolchain ──────────────────────────────────────────────────
if command -v rustup >/dev/null 2>&1; then
  echo "[1/4] rust already installed: $(rustup --version 2>/dev/null || true)"
else
  echo "[1/4] installing rustup (silent)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
    sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path
  . "$HOME/.cargo/env"
fi

# ─── 2) wasm32 target ───────────────────────────────────────────────────
echo "[2/4] adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# ─── 3) Trunk (prebuilt) ────────────────────────────────────────────────
echo "[3/4] installing trunk ${TRUNK_VERSION} (prebuilt, musl-static)..."
mkdir -p ./bin
# musl variant is statically linked — works on any linux regardless of
# glibc version (vercel's image ships an older glibc)
curl -sSL \
  "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-musl.tar.gz" |
  tar -xz -C ./bin
chmod +x ./bin/trunk
export PATH="$PWD/bin:$PATH"
trunk --version

# ─── 4) build ───────────────────────────────────────────────────────────
echo "[4/4] building release wasm bundle..."
trunk build --release

echo "[done] dist/ ready for vercel"
ls -la dist/
