#!/usr/bin/env bash

WORKSPACE=$(cd "$(dirname "$0")" && pwd -P)

# eg:
# sh run.sh https://www.rust-lang.org/ rust.md
docker run --rm -it "$(docker build -q "$WORKSPACE")" "$@"
