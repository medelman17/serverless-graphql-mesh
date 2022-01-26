import {
  Stack,
  StackProps,
  aws_route53 as route53,
  aws_certificatemanager as acm,
  aws_apigateway as apigateway,
  aws_route53_targets as r53targets,
  aws_ecr as ecr,
  CfnOutput,
} from 'aws-cdk-lib'
import { Construct } from 'constructs'

export interface InfraStackProps extends StackProps {
  config: {
    domain: {
      hosted_zone_id: string
      hosted_zone_name: string
      certificate_arn: string
    }
  }
}

export class InfraStack extends Stack {
  readonly apex_domain: route53.IHostedZone
  readonly graph_domain: route53.HostedZone
  readonly apigateway: apigateway.RestApi

  constructor(scope: Construct, id: string, protected props: InfraStackProps) {
    super(scope, id, props)

    this.apex_domain = route53.HostedZone.fromHostedZoneAttributes(
      this,
      'ApexDomain',
      {
        zoneName: this.props.config.domain.hosted_zone_name,
        hostedZoneId: this.props.config.domain.hosted_zone_id,
      },
    )

    this.graph_domain = new route53.PublicHostedZone(this, 'GraphDomain', {
      zoneName: `graph.${this.props.config.domain.hosted_zone_name}`,
    })

    new CfnOutput(this, 'GraphDomainHostedZoneArn', {
      value: this.graph_domain.hostedZoneArn,
      exportName: 'GraphDomainHostedZoneArn',
    })

    new CfnOutput(this, 'GraphDomainHostedZoneId', {
      value: this.graph_domain.hostedZoneId,
      exportName: 'GraphDomainHostedZoneId',
    })

    new CfnOutput(this, 'GraphDomainHostedZoneName', {
      value: this.graph_domain.zoneName,
      exportName: 'GraphDomainHostedZoneName',
    })

    // Create zone delegation record in apex domain

    new route53.ZoneDelegationRecord(this, 'GraphZoneDelegationRecord', {
      nameServers: this.graph_domain.hostedZoneNameServers ?? [],
      zone: this.apex_domain,
      recordName: this.graph_domain.zoneName,
    })

    this.apigateway = new apigateway.RestApi(this, 'RestApi', {
      deploy: true,
      deployOptions: {
        metricsEnabled: true,
        loggingLevel: apigateway.MethodLoggingLevel.INFO,
        dataTraceEnabled: true,
        tracingEnabled: true,
      },
      domainName: {
        domainName: this.graph_domain.zoneName,
        endpointType: apigateway.EndpointType.EDGE,
        certificate: acm.Certificate.fromCertificateArn(
          this,
          'Cert',
          this.props.config.domain.certificate_arn,
        ),
      },
    })

    this.apigateway.root.addMethod(
      'GET',
      new apigateway.HttpIntegration('https://apollographql.com'),
    )

    new route53.ARecord(this, 'AliasRecord', {
      zone: this.graph_domain,
      target: route53.RecordTarget.fromAlias(
        new r53targets.ApiGateway(this.apigateway),
      ),
    })

    new CfnOutput(this, 'ApiGatewayRestApiId', {
      value: this.apigateway.restApiId,
      exportName: 'ApiGatewayRestApiId',
    })

    new CfnOutput(this, 'ApiGatewayRestApiRootResourceId', {
      value: this.apigateway.restApiRootResourceId,
      exportName: 'ApiGatewayRestApiRootResourceId',
    })

    new CfnOutput(this, 'ApiGatewayRestApiName', {
      value: this.apigateway.restApiName,
      exportName: 'ApiGatewayRestApiName',
    })
  }
}
