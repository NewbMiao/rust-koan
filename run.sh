#!/usr/bin/env bash

WORKSPACE=$(cd "$(dirname "$0")" && pwd -P)
RUST_VERSION=1.56.1
WORK_DIR=/usr/src/myapp

docker run -it --rm --user "$(id -u)":"$(id -g)" \
    -v "$WORKSPACE":"$WORK_DIR" -w "$WORK_DIR" \
    -e CARGO_HOME="$WORK_DIR"/.cargo \
    rust:"$RUST_VERSION" cargo run

