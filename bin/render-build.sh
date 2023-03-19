#!/usr/bin/env bash
# exit on error
set -o errexit
cargo build
cargo run