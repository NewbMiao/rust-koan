#! /bin/bash

cargo lambda build --release --arm64 --output-format zip
cp ./target/lambda/lambda-demo/bootstrap.zip ./infrastructure/

# cd infrastructure
# terraform init
# terraform plan
# terraform apply -auto-approve
