#!/bin/bash

# cd lambda || exit 0

docker buildx build \
    --platform=linux/arm64 \
    --output ./lambda/out \
    --file ./lambda/Dockerfile \
    ./lambda
