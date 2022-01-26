import { Construct } from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export interface HttpApiLambdaRustProps {
  // Define construct properties here
}

export class HttpApiLambdaRust extends Construct {

  constructor(scope: Construct, id: string, props: HttpApiLambdaRustProps = {}) {
    super(scope, id);

    // Define construct contents here

    // example resource
    // const queue = new sqs.Queue(this, 'HttpApiLambdaRustQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
