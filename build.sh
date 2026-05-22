#!/usr/bin/env bash
# Vercel build script — bootstraps the Rust toolchain (if not already present)
# and Trunk, then builds the WASM bundle.
#
# Vercel's build image ships with Rust pre-installed at /rust/bin, so we
# detect that and skip the rustup install in that case. Locally / on other
# CI systems without Rust we run the installer.

set -euo pipefail

TRUNK_VERSION=0.21.5

# ─── 1) Rust toolchain ─────────────────────────────────────────────────────
if command -v rustup >/dev/null 2>&1; then
  echo "[1/4] rust already installed: $(rustup --version 2>/dev/null || rustc --version)"
  # source whatever env files exist so PATH is set up correctly
  [ -f /rust/env ] && . /rust/env
  [ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"
else
  echo "[1/4] installing rustup (silent)..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
    sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path
  . "$HOME/.cargo/env"
fi

# ─── 2) wasm32 target ──────────────────────────────────────────────────────
echo "[2/4] adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

# ─── 3) Trunk (prebuilt binary into ./bin) ─────────────────────────────────
echo "[3/4] installing trunk ${TRUNK_VERSION} (prebuilt)..."
mkdir -p ./bin
curl -sSL \
  "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" |
  tar -xz -C ./bin
chmod +x ./bin/trunk
export PATH="$PWD/bin:$PATH"
trunk --version

# ─── 4) build ──────────────────────────────────────────────────────────────
echo "[4/4] building release wasm bundle..."
trunk build --release

echo "[done] dist/ ready for vercel"
ls -la dist/
