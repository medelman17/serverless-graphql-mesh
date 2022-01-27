use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};

use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router,
};

use std::net::SocketAddr;

mod lib;

// basic handler that responds with a static string
async fn root() -> &'static str {
    "RUST ROUTER PLACEHOLDER!"
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    crate::lib::init_tracing();
    let app = Router::new().route("/", get(root));

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
