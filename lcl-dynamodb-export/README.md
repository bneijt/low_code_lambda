# Introduction

Call dyanmodb export table functionality based on eventbridge schedule.

Implements https://docs.rs/aws-sdk-dynamodb/latest/aws_sdk_dynamodb/struct.Client.html#method.export_table_to_point_in_time

Configuration

Use the `LCL_DYNDB_EXPORT_` prefix in environment variables:
- `LCL_DYNDB_EXPORT_TABLE_ARN`: name of Dynamodb table to export
- `LCL_DYNDB_EXPORT_S3_BUCKET`: bucket to export to
- `LCL_DYNDB_EXPORT_S3_PREFIX`: export s3 prefix


## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

```bash
cargo lambda invoke --data-example eventbridge-schedule
```

```json
{
  "version": "0",
  "id": "dbc1c73a-c51d-0c0e-ca61-ab9278974c57",
  "account": "1234567890",
  "time": "2023-05-23T11:38:46Z",
  "region": "us-east-1",
  "detail-type": "OrderPlaced",
  "source": "myapp.orders-service",
  "resources": [],
  "detail": {
    "data": {
      "order_id": "c172a984-3ae5-43dc-8c3f-be080141845a",
      "customer_id": "dda98122-b511-4aaf-9465-77ca4a115ee6",
      "order_total": "120.00"
    }
  }
}
```
