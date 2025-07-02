#!/bin/bash
set -e
PACKAGES="dynamodb-export"

for package in $PACKAGES; do
    echo "Building $package"
    cargo lambda build --release --output-format binary --package "lcl-${package}"
    du -h "target/lambda/lcl-${package}/bootstrap"
    docker build -f Dockerfile \
        --tag "low_code_lambda/${package}" \
        --build-arg "PACKAGE=lcl-${package}" \
        "target/lambda/lcl-${package}"
done
