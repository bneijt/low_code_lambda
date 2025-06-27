#!/bin/bash
set -e
cargo lambda build --release --output-format binary --package "lcl-dynamodb-export"

docker build -f Dockerfile \
    --tag "lcl-dynamodb-export" \
    --build-arg PACKAGE=lcl-dynamodb-export \
    .
