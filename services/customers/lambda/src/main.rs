use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, ID,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use std::net::SocketAddr;
use tracing::info;

mod lib;

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(
    schema: Extension<lib::GraphQLServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    lib::init_tracing();
    let schema = Schema::build(lib::Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .finish();
    let app = Router::new()
        .route("/", get(playground).post(graphql_handler))
        .route("/health", get(root))
        .layer(AddExtensionLayer::new(schema));

    if is_running_on_lambda() {
        info!("We are inside lambda-land");
        run_hyper_on_lambda(app).await?;
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 4001));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}
