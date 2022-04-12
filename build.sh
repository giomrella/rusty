#!/bin/sh
set -e

#build Rust
mkdir -p "output"

#builds a native binary and zip
docker build  -t rust-lambda .


#copy from the docker container to host
containerId=$(docker create -ti rust-lambda bash)
docker cp ${containerId}:function.zip ./output

## Deploy rust lambda

alias sam='sam.cmd'

sam deploy -t aws-sam-template.yaml --no-confirm-changeset --no-fail-on-empty-changeset --stack-name gio-lambda-bucket --s3-bucket aws-lambda-comparison --capabilities CAPABILITY_IAM
