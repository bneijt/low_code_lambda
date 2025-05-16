# Introduction

Call dyanmodb export table functionality based on eventbridge schedule.

Implements https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/struct.Client.html#method.export_table_to_point_in_time

Configuration

Use the `LCL_DYNDB_EXPORT_` prefix in environment variables:
- `LCL_DYNDB_EXPORT_TABLE_NAME`: name of Dynamodb table to export
-


## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

```bash
cargo lambda invoke --data-example eventbridge-schedule
```
