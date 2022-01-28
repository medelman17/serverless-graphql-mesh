pub mod test_data;
pub mod utils;
pub use apollo_router::configuration::Configuration;
use apollo_router::http_subgraph::HttpSubgraphFetcher;
pub use apollo_router::ConfigurationKind;
use apollo_router::FederatedServer;
pub use apollo_router_core::prelude::graphql::*;
use apollo_router_core::prelude::*;
use bytes::Bytes;
pub use utils::environment;
pub use utils::fixtures;

pub async fn create_handler(
    server: FederatedServer,
) -> Result<(), lambda_http::lambda_runtime::Error> {
    let mut server_handle = server.serve();
    let socket = server_handle.ready().await.expect("Server Never Ready");

    lambda_http::lambda_runtime::run(lambda_http::handler(
        move |req: lambda_http::Request, _ctx: lambda_http::Context| {
            let (_parts, body) = req.into_parts();

            let bytes = Bytes::from(body.to_owned());
            let request = graphql::Request::from_bytes(bytes).unwrap();
            return async move {
                let response = HttpSubgraphFetcher::new(
                    "federated",
                    url::Url::parse(&format!("http://{}/graphql", socket)).unwrap(),
                )
                .stream(request)
                .await
                .unwrap();

                lambda_http::IntoResponse::into_response(response.data.as_str().unwrap());
                Ok(())
            };
        },
    ))
    .await
}
