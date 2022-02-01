use aws_lambda::environment;
use lambda_http::{
    run, service_fn, Error as LambdaError, IntoResponse, Request as LambdaRequest, RequestExt,
    Response as LambdaResponse,
};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    if environment::is_lambda() {
        run(service_fn(func)).await?;
    } else {
        println!("Hello from Local");
    }
    Ok(())
}

async fn func(event: LambdaRequest) -> Result<impl IntoResponse, LambdaError> {
    let (parts, body) = event.into_parts();
    println!("METHOD: {:?}", parts.method);
    println!("URI: {:?}", parts.uri);
    println!("HEADERS: {:?}", parts.headers);
    println!("BODY: {:?}", body);
    let resp = LambdaResponse::builder()
        .status(200)
        .body("{}")
        .expect("failed to render response");
    Ok(resp)
}
