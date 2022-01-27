import { Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { aws_lambda_nodejs as lambda, aws_iam as iam, aws_secretsmanager as secrets } from 'aws-cdk-lib'
import { HttpLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha'
import * as apigwv2 from '@aws-cdk/aws-apigatewayv2-alpha'
import { Tracing } from 'aws-cdk-lib/aws-lambda'

export class GatewayStack extends Stack {
  api: apigwv2.HttpApi
  handler: lambda.NodejsFunction
  domain: apigwv2.IDomainName
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props)

    const lambda_role = new iam.Role(this, 'LambdaHandlerRole', {
      assumedBy: new iam.ServicePrincipal('lambda.amazonaws.com'),
    })

    lambda_role.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        'service-role/AWSLambdaBasicExecutionRole',
      ),
    )

    const apolloKeySecret = secrets.Secret.fromSecretNameV2(
      this,
      'apollo-key',
      "apolloKey"
    )

    this.handler = new lambda.NodejsFunction(this, 'Handler', {
      handler: 'handler',
      entry: './lambda/index.ts',
      role: lambda_role,
      tracing: Tracing.ACTIVE,
      environment: {
        APOLLO_GRAPH_REF: "Ocrateris-Serverless-GraphQL@node-gateway",
        APOLLO_KEY: apolloKeySecret.secretValue.toString(),
        APOLLO_OUT_OF_BAND_REPORTER_ENDPOINT: "https://outofbandreporter.api.apollographql.com"
      },
    })

    this.domain = apigwv2.DomainName.fromDomainNameAttributes(this, 'DN', {
      regionalDomainName: 'd-hngc766rjk.execute-api.us-west-2.amazonaws.com',
      regionalHostedZoneId: 'Z2OJLYMUO9EFXC',
      name: 'mesh.ocrateris.cloud',
    })

    this.api = new apigwv2.HttpApi(this, 'NewHttpApi', {
      defaultIntegration: new HttpLambdaIntegration('Gateway', this.handler),
      defaultDomainMapping: {
        domainName: this.domain,
        mappingKey: 'gateway',
      },
    })

    // The code that defines your stack goes here

    // example resource
    // const queue = new sqs.Queue(this, 'GatewayQueue', {
    //   visibilityTimeout: cdk.Duration.seconds(300)
    // });
  }
}
