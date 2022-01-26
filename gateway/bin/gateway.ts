#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { GatewayStack } from '../lib/gateway-stack';

const app = new cdk.App();
new GatewayStack(app, 'GatewayStack', {
  env: {
    account: '661099804654',
    region: 'us-west-2',
  },
});