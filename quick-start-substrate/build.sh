#!/usr/bin/env bash

RUST_LOG=debug RUST_BACKTRACE=1

cd runtime/wasm/
./build.sh

cd ../..
cargo build
