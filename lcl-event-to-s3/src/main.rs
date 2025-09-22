use chrono::Utc;
use lambda_runtime::{Error, run, service_fn};
use tracing_subscriber;

use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use lambda_runtime::{LambdaEvent, tracing};
use serde_json::{Value, to_string};

use std::env::{self};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().init();
    run(service_fn(function_handler)).await
}

pub(crate) async fn function_handler(event: LambdaEvent<Value>) -> Result<String, Error> {
    // Extract some useful information from the request
    tracing::info!("start");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let export_s3_bucket = require_env("LCL_EXPORT_S3_BUCKET")?;
    let export_s3_prefix = require_env("LCL_EXPORT_S3_PREFIX")?;
    let now = Utc::now();
    let partition = format!("{}", now.format("%Y-%m-%d"));

    client
        .put_object()
        .bucket(export_s3_bucket)
        .key(format!(
            "{}/{}/{}.json",
            export_s3_prefix, partition, event.context.request_id
        ))
        .body(
            to_string(&event.into_parts())
                .expect("Should be able to serialize event")
                .into_bytes()
                .into(),
        )
        .send()
        .await
        .context("Failed trying to put event on s3")?;
    // Store tje lambda event as json in s3
    Ok("Ok".to_string())
}

fn require_env(environment_variable_name: &str) -> anyhow::Result<String> {
    env::var(environment_variable_name).context(format!(
        "Missing required environment variable '{}'",
        environment_variable_name
    ))
}
