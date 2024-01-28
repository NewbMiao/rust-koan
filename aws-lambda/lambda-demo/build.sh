#! /bin/bash
# build
cargo lambda build --release --arm64

# pack files into lambda folder
mkdir -p ./infrastructure/lambda
cp ./target/lambda/lambda-demo/bootstrap ./infrastructure/lambda
yarn build
cp -r ./public ./infrastructure/lambda/
cp -r ./templates ./infrastructure/lambda/

# zip files in lambda folder as bootstrap.zip
rm ./infrastructure/bootstrap.zip
cd ./infrastructure/lambda || exit
zip -r ../bootstrap.zip ./*

# deploy
# cd ../
# terraform init
# terraform plan
# terraform apply -auto-approve
