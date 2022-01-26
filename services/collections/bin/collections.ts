#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { CollectionsStack } from '../lib/collections-stack';

const app = new cdk.App();
new CollectionsStack(app, 'CustomersStack', {

});
