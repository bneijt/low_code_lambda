use anyhow::{Context, Result};
use aws_config::{BehaviorVersion, load_defaults};
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{LambdaEvent, tracing};
use std::env;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
pub(crate) async fn function_handler(event: LambdaEvent<CloudWatchEvent>) -> Result<()> {
    // Extract some useful information from the request
    tracing::info!("Starting");
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let table_name = env::var("LCL_EXPORT_TABLE_NAME")
        .context("Environment variable LCL_EXPORT_TABLE_NAME not set")?;
    let export_arn = format!("arn:aws:s3:::table-exports");

    let resp = client
        .export_table_to_point_in_time()
        .table_arn(&table_name)
        .s3_bucket(&export_arn)
        .send()
        .await
        .context("Failed to initiate table export")?;

    match resp.export_description {
        Some(description) => {
            println!("Export initiated successfully!");
            println!("Export ARN: {:?}", description.export_arn);
            println!("Export status: {:?}", description.export_status);
        }
        None => {
            println!("Failed to initiate export.");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_event_handler() {
        let event = LambdaEvent::new(CloudWatchEvent::default(), Context::default());
        let response = function_handler(event).await.unwrap();
        assert_eq!((), response);
    }
}
