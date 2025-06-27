#!/bin/bash
set -e
PACKAGES="dynamodb-export"

for package in $PACKAGES; do
    echo "Building $package"
    RUSTFLAGS='-C target-feature=+crt-static' cargo lambda build --release --output-format binary --package "lcl-${package}"
    docker build -f Dockerfile \
        --tag "low_code_lambda/${package}" \
        --build-arg "PACKAGE=lcl-${package}" \
        "target/lambda/lcl-${package}"
done
