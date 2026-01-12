#!/bin/bash

set -e
mkdir -p temp

cargo expand --all-features --test mod validation::options >> temp/output.rs
cargo check --all-features --test mod
