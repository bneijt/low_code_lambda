Low Code Lambda
====
[Low code lambda](https://github.com/lambdelta-dev/low-code-lambda) are Rust based AWS Lambda containers that are ready to use for small orchestration tasks.

They are configured via environment variables.

# Implementations

- [DynamoDB export](lcl-dynamodb-export): trigger dynamodb export to S3 based on eventbridge trigger
- [Event to S3](lcl-event-to-s3): any event hitting the lambda is stored as an S3 json file for later analysis.

Any feature requests, or pull requests are welcome. Feel free to [file an issue](https://github.com/lambdelta-dev/low-code-lambda/issues).


## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

You can run regular Rust unit tests with `cargo test`.

If you want to run integration tests locally, you can use the `cargo lambda watch` and `cargo lambda invoke` commands to do it.

First, run `cargo lambda watch` to start a local server. When you make changes to the code, the server will automatically restart.

Second, you'll need a way to pass the event data to the lambda function.

You can use the existent [event payloads](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) in the Rust Runtime repository if your lambda function is using one of the supported event types.

You can use those examples directly with the `--data-example` flag, where the value is the name of the file in the [lambda-events](https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/lambda-events/src/fixtures) repository without the `example_` prefix and the `.json` extension.

```bash
cargo lambda invoke --data-example eventbridge-schedule
```

Read more about running the local server in [the Cargo Lambda documentation for the `watch` command](https://www.cargo-lambda.info/commands/watch.html).
Read more about invoking the function in [the Cargo Lambda documentation for the `invoke` command](https://www.cargo-lambda.info/commands/invoke.html).

## Deploying

The main distribution channel are docker containers from docker-hub. This allows you to simply take the binary from there and either copy the container to your own ECR repository or deploy it as a lambda zip by taking it from the container.

### Using cargo lambda
To deploy the project, run `cargo lambda deploy`. This will create an IAM role and a Lambda function in your AWS account.

Read more about deploying your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/deploy.html).

### Using CDK

Using typescript CDK you can deploy the code by taking the runtime from the docker image and publishing it as a zip in your lambda.

For example:
```typescript
import * as cdk from "aws-cdk-lib";
import * as dynamodb from "aws-cdk-lib/aws-dynamodb";
import * as iam from "aws-cdk-lib/aws-iam";
import * as ec2 from "aws-cdk-lib/aws-ec2";
import * as lambda from "aws-cdk-lib/aws-lambda";
import * as logs from "aws-cdk-lib/aws-logs";
import * as sns from "aws-cdk-lib/aws-sns";
import * as s3 from "aws-cdk-lib/aws-s3";
import * as events from "aws-cdk-lib/aws-events";
import * as targets from "aws-cdk-lib/aws-events-targets";
import * as cloudwatch from "aws-cdk-lib/aws-cloudwatch";
import * as actions from "aws-cdk-lib/aws-cloudwatch-actions";

const exportLambda = new lambda.Function(this, "DynamoDBExport", {
      functionName: "tigger-export",
      description: "Trigger table export",
      code: lambda.Code.fromAsset("../dist", {
        bundling: {
          image: cdk.DockerImage.fromRegistry(
            "bneijt/lcl-dynamodb-export:v0.0.1",
          ),
          entrypoint: ["/bin/sh"],
          user: "0:0",
          command: ["-c", "cp /var/runtime/* /asset-output/"],
        },
      }),
      runtime: lambda.Runtime.PROVIDED_AL2023,
      handler: "not_used",
      retryAttempts: 0,
      memorySize: 128,
      timeout: cdk.Duration.seconds(10),
      environment: {
        LCL_DYNDB_EXPORT_TABLE_ARN: dynamodbTable.tableArn,
        LCL_DYNDB_EXPORT_S3_BUCKET: targetBucket.bucketName,
        LCL_DYNDB_EXPORT_S3_PREFIX: `table_exports/`,
        LCL_DYNDB_EXPORT_TYPE: "FULL_EXPORT",
        LCL_DYNDB_EXPORT_FORMAT: "DYNAMODB_JSON",
      },
      loggingFormat: lambda.LoggingFormat.JSON,
      systemLogLevelV2: lambda.SystemLogLevel.WARN,
    });

exportLambda.role?.addToPrincipalPolicy(
    new iam.PolicyStatement({
    actions: ["dynamodb:ExportTableToPointInTime"],
    resources: [dynamodbTable.tableArn],
    }),
);
exportLambda.role?.addToPrincipalPolicy(
    new iam.PolicyStatement({
    actions: ["s3:AbortMultipartUpload", "s3:PutObject", "s3:PutObjectAcl"],
    resources: [targetBucket.arnForObjects("*")],
    }),
);

new events.Rule(this, "PvsExportScheduleRule", {
    ruleName: "tas-pvs-dynamodb-export",
    description: "Trigger export of TAS PVS DynamoDB table",
    schedule: events.Schedule.cron({
    minute: "8",
    hour: "9",
    day: "1/3",
    month: "*",
    year: "*",
    }),
    targets: [new targets.LambdaFunction(exportLambda)],
});

```

### During CI/CD extract the binary from the container

You can also extract the bootstrap binary from the container during CI/CD:

```
docker create --name lclcp bneijt/lcl-dynamodb-export:v0.0.1
docker cp lclcp:/var/runtime/bootstrap bootstrap
docker rm lclcp
```

and after that deploy the binary with, for example, terraform:

```
data "archive_file" "lclzip" {
  type = "zip"
  source_file = "bootstrap"
  output_path = "lambda_function.zip"
}

resource "aws_lambda_function" "my_dynamodb_stream_lambda" {
  function_name = "my-dynamodb-stream-lambda"
  handler = "not_used"
  filename = data.archive_file.lclzip.output_path
  source_code_hash = data.archive_file.lclzip.output_base64sha256
  runtime = "provided.al2023"
}
```


### Using docker container

You can copy the container to your ecr, then deploy it from the ECR.

```
docker pull bneijt/lcl-dynamodb-export:v0.0.1
docker tag bneijt/lcl-dynamodb-export:v0.0.1 "${AWS_ACCOUNT}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_NAME}:latest"
docker push "${AWS_ACCOUNT}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_NAME}:latest"
```

