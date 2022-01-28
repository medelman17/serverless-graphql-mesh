use apollo_router::configuration::Configuration;
use apollo_router::http_subgraph::HttpSubgraphFetcher;
use apollo_router::{FederatedServer, FederatedServerHandle, SchemaKind, ShutdownKind};
use apollo_router_core::{prelude::graphql, Fetcher};
use lambda_http::{
    handler,
    lambda_runtime::{self, Error},
    Body as LambdaBody, Context as LambdaContext, Request as LambdaRequest, RequestExt,
};
use lambda_router::{environment, fixtures, ConfigurationKind};
use serde::Deserialize;
use std::net::SocketAddr;

use serde_json::to_string_pretty;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let configuration = ConfigurationKind::Instance(Configuration::builder().build().boxed());
    let schema = SchemaKind::Instance(Box::new(fixtures::get_sample_schema().parse().unwrap()));
    let server: FederatedServer = FederatedServer::builder()
        .configuration(configuration)
        .schema(schema)
        .shutdown(ShutdownKind::None)
        .build();

    if environment::is_lambda() {
        lambda_router::create_handler(server).await;
    } else {
        let mut server_handle = server.serve();

        if let Err(err) = server_handle.await {
            // tracing::error!("{}", err.into());
            return Err(err.into());
        }
    }
    Ok(())
}
