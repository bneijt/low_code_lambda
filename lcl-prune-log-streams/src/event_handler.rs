use anyhow::{Context, Result};
use aws_config::BehaviorVersion;
use aws_lambda_events::event::cloudwatch_events::CloudWatchEvent;
use aws_sdk_cloudwatchlogs::Client as LogsClient;
use lambda_runtime::{Error, LambdaEvent, tracing};

use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) async fn function_handler(_: LambdaEvent<CloudWatchEvent>) -> Result<String, Error> {
    tracing::info!("start");
    tracing::info!("start prune log streams");

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let logs_client = LogsClient::new(&config);

    let log_groups_env = require_env("LCL_LOG_GROUPS")?;
    let log_groups: Vec<&str> = log_groups_env
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    // Read keep-at-least value from env, default to 5
    let keep_at_least: usize = match env::var("LCL_KEEP_AT_LEAST") {
        Ok(val) => val.parse().unwrap_or(5),
        Err(_) => 5,
    };

    for log_group in log_groups {
        tracing::info!("Processing log group: {}", log_group);

        // Get log group details (to fetch retention period)
        let describe_resp = logs_client
            .describe_log_groups()
            .log_group_name_prefix(log_group)
            .send()
            .await
            .context("Failed to describe log groups")?;

        let group = describe_resp.log_groups.as_ref().and_then(|groups| {
            groups
                .iter()
                .find(|g| g.log_group_name.as_deref() == Some(log_group))
        });

        let retention_days = match group.and_then(|g| g.retention_in_days) {
            Some(days) => days,
            None => {
                tracing::warn!(
                    "No retention period set for log group {}, skipping",
                    log_group
                );
                continue;
            }
        };

        let retention_secs = retention_days as u64 * 24 * 60 * 60;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Gather all log streams for this group, sorted by last event time descending
        let mut all_streams = Vec::new();
        let mut next_token = None;
        loop {
            let mut req = logs_client
                .describe_log_streams()
                .log_group_name(log_group)
                .order_by(aws_sdk_cloudwatchlogs::types::OrderBy::LastEventTime)
                .descending(true)
                .limit(50);

            if let Some(token) = &next_token {
                req = req.next_token(token);
            }

            let streams_resp = req.send().await.context("Failed to describe log streams")?;
            let mut streams = streams_resp.log_streams.unwrap_or_default();
            all_streams.append(&mut streams);

            next_token = streams_resp.next_token;
            if next_token.is_none() {
                break;
            }
        }

        // Sort streams by last event time descending (most recent first)
        all_streams.sort_by(|a, b| {
            let a_time = a.last_event_timestamp.unwrap_or(0);
            let b_time = b.last_event_timestamp.unwrap_or(0);
            b_time.cmp(&a_time)
        });

        // Only consider deleting streams after the N most recent ones
        for (idx, stream) in all_streams.iter().enumerate() {
            let stream_name = match &stream.log_stream_name {
                Some(name) => name,
                None => continue,
            };

            if idx < keep_at_least {
                tracing::info!("Keeping recent stream '{}'", stream_name);
                continue;
            }

            let last_event_ts = stream.last_event_timestamp.unwrap_or(0) as u64 / 1000;
            let last_ingestion_ts = stream.last_ingestion_time.unwrap_or(0) as u64 / 1000;

            let is_expired =
                now > last_event_ts + retention_secs && now > last_ingestion_ts + retention_secs;

            // Check if stream is empty (no stored bytes or no events)
            let stored_bytes = stream.stored_bytes.unwrap_or(0);
            let is_empty = stored_bytes == 0;

            if is_expired && is_empty {
                tracing::info!(
                    "Deleting expired and empty log stream: {} in group {}",
                    stream_name,
                    log_group
                );
                let _ = logs_client
                    .delete_log_stream()
                    .log_group_name(log_group)
                    .log_stream_name(stream_name)
                    .send()
                    .await;
            }
        }
    }

    Ok("Pruned log streams".to_string())
}

fn require_env(environment_variable_name: &str) -> anyhow::Result<String> {
    env::var(environment_variable_name).context(format!(
        "Missing required environment variable '{}'",
        environment_variable_name
    ))
}
