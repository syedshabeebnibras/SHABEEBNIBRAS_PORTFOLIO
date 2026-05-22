#!/usr/bin/env bash
# Vercel build script — bootstraps Rust toolchain + Trunk, then builds.
# Kept as a separate file because vercel.json's buildCommand has a 256-char limit.

set -euo pipefail

TRUNK_VERSION=0.21.5

echo "[1/4] installing rustup (silent)..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs |
  sh -s -- -y --default-toolchain stable --profile minimal --no-modify-path
. "$HOME/.cargo/env"

echo "[2/4] adding wasm32-unknown-unknown target..."
rustup target add wasm32-unknown-unknown

echo "[3/4] installing trunk ${TRUNK_VERSION} (prebuilt binary)..."
mkdir -p "$HOME/.cargo/bin"
curl -sSL \
  "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" |
  tar -xz -C "$HOME/.cargo/bin"
chmod +x "$HOME/.cargo/bin/trunk"

echo "[4/4] building release wasm bundle..."
trunk build --release

echo "[done] dist/ ready for vercel"
ls -la dist/
