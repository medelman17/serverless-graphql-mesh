#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { ReviewsStack } from '../lib/reviews-stack';

const app = new cdk.App();
new ReviewsStack(app, 'CustomersStack', {

});
