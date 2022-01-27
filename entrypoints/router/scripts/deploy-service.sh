#!/usr/bin/env bash

NODE_ENV=production

cdk synth
cdk deploy \
  --outputs-file