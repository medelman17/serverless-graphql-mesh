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

use crate::lib::{Customer, Product, Review};

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
    let reviews = vec![
        lib::Review {
            body: "A highly effective form of birth control.".into(),
            author: Customer { id: "1234".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
        Review {
            body: "Fedoras are one of the most fashionable hats around and can look great with a variety of outfits.".into(),
            author: Customer { id: "1234".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
        Review {
            body: "This is the last straw. Hat you will wear. 11/10".into(),
            author: Customer { id: "7777".into() },
            product: Product {
                upc: "top-1".to_string(),
            },
        },
    ];
    let schema = Schema::build(lib::Query, EmptyMutation, EmptySubscription)
        .data(reviews)
        .enable_federation()
        .finish();
    let app = Router::new()
        .route("/", get(playground).post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    if is_running_on_lambda() {
        run_hyper_on_lambda(app).await?;
    } else {
        let addr = SocketAddr::from(([0, 0, 0, 0], 4001));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}
