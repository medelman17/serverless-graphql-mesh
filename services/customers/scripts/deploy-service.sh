#!/usr/bin/env bash


cdk synth
cdk deploy \
  --outputs-file ../../config/customers_service-outputs.json