#!/usr/bin/env bash

SERVICE_TARGET_ARCHITECTURE=${1:-aarch64-unknown-linux-gnu}

PACKAGE_DIR="$PWD"
SCRIPTS_DIR="$PACKAGE_DIR/scripts"

export CC="$SCRIPTS_DIR/zcc"
export CXX="$SCRIPTS_DIR/zxx"
CARGO_TRIPLE="$(echo ${SERVICE_TARGET_ARCHITECTURE^^} | tr '-' '_')"
  CARGO_LINKER="CARGO_TARGET_${CARGO_TRIPLE}_LINKER" && \
   printf -v $CARGO_LINKER "$SERVICE_TARGET_ARCHITECTURE-gcc" && \
   export $CARGO_LINKER

cargo build \
  --target "$SERVICE_TARGET_ARCHITECTURE" \
  --release