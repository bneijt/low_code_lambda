#!/bin/bash
set -e
cd lcl-dynamodb-export
docker build -f ../Dockerfile \
    --platform linux/amd64  \
    --build-arg PACKAGE=lcl-dynamodb-export \
    .
