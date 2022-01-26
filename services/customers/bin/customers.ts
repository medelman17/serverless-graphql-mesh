#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { CustomersStack } from '../lib/customers-stack'

const app = new cdk.App()
new CustomersStack(app, 'CustomersStack', {
  env: {
    account: '661099804654',
    region: 'us-west-2',
  },
})
