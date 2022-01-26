#!/bin/bash

WORK_DIR="$PWD"
ASSET_DIR="$WORK_DIR/assets"
OUT_DIR="$WORK_DIR/lambda/out"

mkdir $ASSET_DIR

cd $OUT_DIR || exit 1

zip -r -X "./lambda.zip" "./bootstrap"
cp "lambda.zip" "$ASSET_DIR"
rm -rf "$OUT_DIR"
