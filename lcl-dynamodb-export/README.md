# Introduction

Call dyanmodb export table functionality based on eventbridge schedule.

## Building

To build the project for production, run `cargo lambda build --release`. Remove the `--release` flag to build for development.

Read more about building your lambda function in [the Cargo Lambda documentation](https://www.cargo-lambda.info/commands/build.html).

## Testing

```bash
cargo lambda invoke --data-example eventbridge-schedule
```
