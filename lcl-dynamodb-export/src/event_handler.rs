use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{Error, LambdaEvent, tracing};

use std::env::{self};

pub(crate) async fn function_handler(_: LambdaEvent<CloudWatchEvent>) -> Result<String, Error> {
    // Extract some useful information from the request
    tracing::info!("start");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let table_arn = require_env("LCL_DYNDB_EXPORT_TABLE_ARN")?;
    let export_s3_bucket = require_env("LCL_DYNDB_EXPORT_S3_BUCKET")?;
    let export_s3_prefix = require_env("LCL_DYNDB_EXPORT_S3_PREFIX")?;

    let resp = client
        .export_table_to_point_in_time()
        .table_arn(&table_arn)
        .s3_bucket(&export_s3_bucket)
        .s3_prefix(&export_s3_prefix)
        .export_format(aws_sdk_dynamodb::types::ExportFormat::DynamodbJson)
        .export_type(aws_sdk_dynamodb::types::ExportType::FullExport)
        .send()
        .await
        .context("Failed to initiate table export")?;

    match resp.export_description {
        Some(description) => {
            tracing::info!("Export initiated successfully.");
            tracing::info!("Export ARN: {:?}", description.export_arn);
            tracing::info!("Export status: {:?}", description.export_status);
        }
        None => {
            tracing::error!("Failed to initiate export");
        }
    }
    Ok("Tried".to_string())
}

fn require_env(environment_variable_name: &str) -> anyhow::Result<String> {
    env::var(environment_variable_name).context(format!(
        "Missing required environment variable '{}'",
        environment_variable_name
    ))
}
