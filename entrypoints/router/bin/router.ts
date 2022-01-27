#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { RouterStack } from '../lib/router-stack'

const app = new cdk.App()
new RouterStack(app, 'RouterStack', {
  env: {
    account: '661099804654',
    region: 'us-west-2',
  },
})
