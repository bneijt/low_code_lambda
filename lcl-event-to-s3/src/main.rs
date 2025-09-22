use chrono::Utc;
use lambda_runtime::{Error, run, service_fn};
use tracing_subscriber;

use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use lambda_runtime::{LambdaEvent, tracing};
use serde::Serialize;
use serde_json::{Value, to_string};

use std::env::{self};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json().init();
    run(service_fn(function_handler)).await
}

#[derive(Serialize)]
struct LambdaEventRecord {
    context: lambda_runtime::Context,
    payload: Value,
}

pub(crate) async fn function_handler(event: LambdaEvent<Value>) -> Result<String, Error> {
    tracing::info!("start");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let export_s3_bucket = require_env("LCL_EXPORT_S3_BUCKET")?;
    let export_s3_prefix = require_env("LCL_EXPORT_S3_PREFIX")?;
    let now = Utc::now();
    let partition = format!("{}", now.format("%Y-%m-%d"));

    let event_record = LambdaEventRecord {
        context: event.context.clone(),
        payload: event.payload.clone(),
    };

    client
        .put_object()
        .bucket(export_s3_bucket)
        .key(format!(
            "{}/{}/{}.json",
            export_s3_prefix, partition, event.context.request_id
        ))
        .body(
            to_string(&event_record)
                .expect("Should be able to serialize event")
                .into_bytes()
                .into(),
        )
        .send()
        .await
        .context("Failed trying to put event on s3")?;
    Ok("{\"status\":\"ok\"}".to_string())
}

fn require_env(environment_variable_name: &str) -> anyhow::Result<String> {
    env::var(environment_variable_name).context(format!(
        "Missing required environment variable '{}'",
        environment_variable_name
    ))
}
