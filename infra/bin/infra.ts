#!/usr/bin/env node
import 'source-map-support/register'
import * as cdk from 'aws-cdk-lib'
import { InfraStack } from '../lib/infra-stack'

const app = new cdk.App()
new InfraStack(app, 'InfraStack', {
  config: {
    domain: {
      hosted_zone_id: app.node.tryGetContext('apex_hosted_zone_id'),
      hosted_zone_name: app.node.tryGetContext('apex_hosted_zone_name'),
      certificate_arn: app.node.tryGetContext('certificate_arn'),
    },
  },
  env: {
    account: app.node.tryGetContext('account'),
    region: app.node.tryGetContext('region'),
  },
})
