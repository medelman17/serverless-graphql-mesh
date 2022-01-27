#!/usr/bin/env bash

SERVICE_TARGET_ARCHITECTURE=${1:-aarch64-unknown-linux-gnu}

PACKAGE_DIR="$PWD"
SERVICE_DIR="$PACKAGE_DIR/lambda"
ARTIFACT_DIR="$SERVICE_DIR/target/$SERVICE_TARGET_ARCHITECTURE/release"
ASSET_DIR="$PACKAGE_DIR/assets"


echo "[Cleaning]"
echo "[Cleaning] -> Remove Existing Assets" && rm -rf "$ASSET_DIR"

echo "[Building]"
echo "[Building] -> Preparing Environment"
  export CC="$SERVICE_DIR/zcc"
  export CXX="$SERVICE_DIR/zxx"
  export RUSTC_WRAPPER=/Users/michael/.cargo/bin/sccache
  CARGO_TRIPLE="$(echo ${SERVICE_TARGET_ARCHITECTURE^^} | tr '-' '_')"
  CARGO_LINKER="CARGO_TARGET_${CARGO_TRIPLE}_LINKER" && \
   printf -v $CARGO_LINKER "$SERVICE_TARGET_ARCHITECTURE-gcc" && \
   export $CARGO_LINKER

echo "[Building] -> Architecture: $SERVICE_TARGET_ARCHITECTURE" && \
  cargo build \
    --manifest-path "$SERVICE_DIR/Cargo.toml" \
    --target "$SERVICE_TARGET_ARCHITECTURE" \
    --message-format short \
    --release

echo "[Packaging]"
echo "[Packaging] -> Archiving Asset" && \
  mkdir $ASSET_DIR && \
  cd $ARTIFACT_DIR || exit && \
  zip -r -X -q "./lambda.zip" "./bootstrap" && \
  cp "lambda.zip" "$ASSET_DIR"
echo "[Packaging] -> File Size: $(stat -f%z "$ASSET_DIR/lambda.zip")"



