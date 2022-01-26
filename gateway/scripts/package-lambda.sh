#!/bin/bash

mkdir assets
zip -r -X "./assets/lambda.zip" "$PWD/lambda/out/bootstrap"
rm -rf "$PWD/lambda/out"
