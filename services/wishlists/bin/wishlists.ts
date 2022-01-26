#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from 'aws-cdk-lib';
import { WishlistStack } from '../lib/wishlist-stack';

const app = new cdk.App();
new WishlistStack(app, 'CustomersStack', {

});
