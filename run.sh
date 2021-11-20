#!/usr/bin/env bash

WORKSPACE=$(cd "$(dirname "$0")" && pwd -P)

# eg:
# SRC_DIR=scrape_url sh run.sh https://www.rust-lang.org/ rust.md
docker run --rm -it "$(docker build --build-arg SRC_DIR="$SRC_DIR" -q "$WORKSPACE" -t "rust-$SRC_DIR")" "$@"
