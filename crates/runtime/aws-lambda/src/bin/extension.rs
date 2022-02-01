use apollo_router::configuration::Configuration;
use apollo_router::{ConfigurationKind, FederatedServer, ShutdownKind, State as RouterState};
use apollo_router_core::Schema;
use aws_lambda::sdk::config::ConfigurationProvider;
use aws_lambda::sdk::schema::SchemaProvider;
use aws_sdk_s3::{Client as S3Client, Error as S3Error};
use bytes::{Buf, Bytes, BytesMut};
use clap::Parser;
use futures::future::{self};
use futures::prelude::*;
use lambda_extension::{service_fn, Error as ExtensionError, LambdaEvent, NextEvent, Runtime};
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

async fn my_extension(event: LambdaEvent) -> Result<(), ExtensionError> {
    match event.next {
        NextEvent::Shutdown(_e) => {
            // do something with the shutdown event
        }
        _ => {
            // ignore any other event
            // because we've registered the extension
            // only to receive SHUTDOWN events
        }
    }
    Ok(())
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Log level
    #[clap(short, long, default_value_t = String::from("info"))]
    log_level: String,

    #[clap(short = 'b', long = "bucket", default_value_t = String::from("ocrateris-sls-router-config"))]
    s3_bucket: String,

    #[clap(short, long = "config", default_value_t = String::from("config.yaml"))]
    configuration_key: String,

    #[clap(short, long = "supergraph", default_value_t = String::from("supergraph.graphql"))]
    supergraph_key: String,
}

fn init_tracing() {
    let args: Args = Args::parse();
    let env_filter = std::env::var("RUST_LOG")
        .ok()
        .unwrap_or(args.log_level.to_string());
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::try_new(&env_filter).expect("ould not get get env"))
        .init();

    // tracing::subscriber::set_global_default(builder.finish());
}

async fn get_router_config(client: &S3Client) -> Configuration {
    let args: Args = Args::parse();
    let bucket = args.s3_bucket;
    let key = args.configuration_key;
    let provider = ConfigurationProvider::from_bucket(client, bucket, key).await;
    let config: Configuration = provider.into();
    return config;
}

async fn get_router_schema(client: &S3Client) -> Schema {
    let args: Args = Args::parse();
    use aws_lambda::sdk::schema::SchemaProvider;
    let bucket = args.s3_bucket;
    let key = args.supergraph_key;
    let provider = SchemaProvider::from_bucket(client, bucket.to_string(), key.to_string()).await;
    let schema: Schema = provider.into();
    return schema;
}

async fn init_remote_provider() -> S3Client {
    let config = aws_config::load_from_env().await;
    let client = S3Client::new(&config);
    return client;
}

async fn router_as_lambda_extension() -> Result<(), ExtensionError> {
    let s3_client = init_remote_provider().await;
    let config = get_router_config(&s3_client).await;
    let schema = get_router_schema(&s3_client).await;
    let server: FederatedServer = FederatedServer::builder()
        .configuration(config)
        .schema(schema)
        .shutdown(ShutdownKind::None)
        .build();

    let mut server_handle = server.serve();

    server_handle
        .state_receiver()
        .for_each(|state: RouterState| {
            match state {
                RouterState::Startup => {
                    tracing::info!(
                        r#"Starting Apollo Router
    *******************************************************************
    ⚠️  Experimental software, not YET recommended for production use ⚠️
    *******************************************************************"#
                    )
                }
                RouterState::Stopped => {
                    tracing::info!("Stopped")
                }
                RouterState::Running { address, .. } => {
                    tracing::info!("Listening on http://{} 🚀", address)
                }
                RouterState::Errored => {
                    tracing::info!("Stopped with error")
                }
            }
            future::ready(())
        })
        .await;

    if let Err(err) = server_handle.await {
        tracing::error!("{:?}", err);
        return Err(err.into());
    };

    Ok(())

    // let rt = Runtime::builder()
    //     .with_events(&["SHUTDOWN"])
    //     .register()
    //     .await?;

    // let func = service_fn(my_extension);
    // lambda_extension::run(func).await
}

async fn router_as_local_server() -> Result<(), ExtensionError> {
    // let args: Args = Args::parse();
    let s3_client = init_remote_provider().await;
    let config = get_router_config(&s3_client).await;
    let schema = get_router_schema(&s3_client).await;
    let server: FederatedServer = FederatedServer::builder()
        .configuration(config)
        .schema(schema)
        .shutdown(ShutdownKind::CtrlC)
        .build();

    let mut server_handle = server.serve();

    server_handle
        .state_receiver()
        .for_each(|state: RouterState| {
            match state {
                RouterState::Startup => {
                    tracing::info!(
                        r#"Starting Apollo Router
*******************************************************************
⚠️  Experimental software, not YET recommended for production use ⚠️
*******************************************************************"#
                    )
                }
                RouterState::Stopped => {
                    tracing::info!("Stopped")
                }
                RouterState::Running { address, .. } => {
                    tracing::info!("Listening on http://{} 🚀", address)
                }
                RouterState::Errored => {
                    tracing::info!("Stopped with error")
                }
            }
            future::ready(())
        })
        .await;

    if let Err(err) = server_handle.await {
        tracing::error!("{:?}", err);
        return Err(err.into());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), ExtensionError> {
    init_tracing();

    if aws_lambda::environment::is_lambda() {
        println!("Lambda Environment Detected");
        router_as_lambda_extension();
    } else {
        println!("Local Environment Detected");
        router_as_local_server().await.unwrap();
    }
    Ok(())
}
