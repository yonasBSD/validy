#!/bin/bash

set -e
mkdir -p temp

cargo expand --lib --tests --all-features tests::derive >> temp/output.rs
cargo check --lib --tests --all-features
