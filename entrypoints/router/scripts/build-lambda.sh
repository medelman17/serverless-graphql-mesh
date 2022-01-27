#!/bin/bash

# cd lambda || exit 0

# docker buildx build \
#     --platform=linux/amd64 \
#     --output ./lambda/out \
#     --file ./lambda/Dockerfile \
#     ./lambda

cd lambda || exit 1

CC=zcc cargo build --target aarch64-unknown-linux-gnu --release

cd .. || exit 1
