import { Stack, StackProps } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { aws_lambda as lambda, aws_iam as iam } from 'aws-cdk-lib'
import { HttpLambdaIntegration } from '@aws-cdk/aws-apigatewayv2-integrations-alpha'
import * as apigwv2 from '@aws-cdk/aws-apigatewayv2-alpha'

export class ReviewsStack extends Stack {
  api: apigwv2.HttpApi
  handler: lambda.Function
  domain: apigwv2.IDomainName

  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const lambda_role = new iam.Role(this, 'LambdaHandlerRole', {
      assumedBy: new iam.ServicePrincipal('lambda.amazonaws.com'),
    })

    lambda_role.addManagedPolicy(
      iam.ManagedPolicy.fromAwsManagedPolicyName(
        'service-role/AWSLambdaBasicExecutionRole',
      ),
    )

    this.handler = new lambda.Function(this, 'LambdaHandlerFunction', {
      code: lambda.AssetCode.fromAsset('./assets/lambda.zip'),
      runtime: lambda.Runtime.PROVIDED_AL2,
      handler: 'not.required',
      tracing: lambda.Tracing.PASS_THROUGH,
      architecture: lambda.Architecture.ARM_64,
      role: lambda_role,
      insightsVersion: lambda.LambdaInsightsVersion.VERSION_1_0_119_0,
      environment: {},
    })

    this.domain = apigwv2.DomainName.fromDomainNameAttributes(this, 'DN', {
      regionalDomainName: 'd-hngc766rjk.execute-api.us-west-2.amazonaws.com',
      regionalHostedZoneId: 'Z2OJLYMUO9EFXC',
      name: 'mesh.ocrateris.cloud',
    })

    this.api = new apigwv2.HttpApi(this, 'NewHttpApi', {
      defaultIntegration: new HttpLambdaIntegration('Reviews', this.handler),
      defaultDomainMapping: {
        domainName: this.domain,
        mappingKey: 'service/reviews',
      },
    })

  }
}
