#!/usr/bin/env bash

ROOT="$(git rev-parse --show-toplevel)"
pushd "${ROOT}" > /dev/null || exit 1

cargo build
cargo run -p uniffi-bindgen generate --language kotlin --out-dir out --library target/debug/liblib.dylib --crate crate_1