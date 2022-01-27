#!/usr/bin/env bash

PACKAGE_DIR="$PWD"
SERVICE_DIR="$PACKAGE_DIR/lambda"
ARTIFACTS_DIR="$PACKAGE_DIR/artifacts"
ASSETS_DIR="$PACKAGE_DIR/assets"

echo "[Cleaning]"

echo "[Cleaning]: Remove Existing Artifacts" && rm -rf "$ARTIFACTS_DIR"
echo "[Cleaning]: Remove Existing Assets" && rm -rf "$ASSETS_DIR"

