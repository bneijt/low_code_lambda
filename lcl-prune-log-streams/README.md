# lcl-prune-log-streams

## Introduction

This AWS Lambda function prunes (deletes) empty and expired CloudWatch log streams from specified log groups, based on a scheduled EventBridge trigger.

It is inspired by the AWS blog post: [Delete empty CloudWatch log streams](https://aws.amazon.com/blogs/mt/delete-empty-cloudwatch-log-streams/).

### What it does

- For each log group in a comma-separated list, it:
  - Keeps the N most recent log streams (default: 5, configurable).
  - Deletes log streams that are both:
    - Empty (no stored bytes).
    - Expired (last event and ingestion time are older than the log group's retention period).

## Configuration

Set the following environment variables:

- `LCL_LOG_GROUPS`: Comma-separated list of CloudWatch log group names to prune.
- `LCL_KEEP_AT_LEAST` (optional): Minimum number of most recent log streams to always keep per log group (default: 5).

Example:

```
LCL_LOG_GROUPS=/aws/lambda/my-func-prod,/aws/lambda/my-func-dev
LCL_KEEP_AT_LEAST=10
```

## Building

To build the project for production, run:

```bash
cargo lambda build --release
```

Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can test locally with:

```bash
cargo lambda watch
```

Or invoke with a sample EventBridge event:

```bash
cargo lambda invoke --data-example eventbridge-schedule
```

Example event payload:

```json
{
  "version": "0",
  "id": "dbc1c73a-c51d-0c0e-ca61-ab9278974c57",
  "account": "1234567890",
  "time": "2023-05-23T11:38:46Z",
  "region": "us-east-1",
  "detail-type": "Scheduled Event",
  "source": "aws.events",
  "resources": [],
  "detail": {}
}
```

## IAM Permissions

The Lambda function requires the following AWS permissions:

- `logs:DescribeLogGroups`
- `logs:DescribeLogStreams`
- `logs:DeleteLogStream`

Grant these permissions to the Lambda execution role for the log groups you wish to prune.

---