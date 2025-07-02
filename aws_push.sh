#!/bin/bash
set -e

PACKAGES="dynamodb-export"
aws ecr-public get-login-password --region us-east-1 | docker login --username AWS --password-stdin public.ecr.aws/n0i9l8j9/low_code_lambda

for package in $PACKAGES; do
    echo "Pushing $package"
    docker tag "low_code_lambda/${package}" public.ecr.aws/n0i9l8j9/low_code_lambda/${package}
    docker push public.ecr.aws/n0i9l8j9/low_code_lambda/${package}
done
