#!/bin/bash

set -e
mkdir -p temp

cargo expand --all-features --test mod validation::allowlist >> temp/allowlist.rs || true
cargo expand --all-features --test mod validation::blocklist >> temp/blocklist.rs || true
cargo expand --all-features --test mod validation::contains >> temp/contains.rs || true
cargo expand --all-features --test mod validation::email >> temp/email.rs || true
cargo expand --all-features --test mod validation::ip >> temp/ip.rs || true
cargo expand --all-features --test mod validation::ipv4 >> temp/ivp4.rs || true
cargo expand --all-features --test mod validation::ipv6 >> temp/ipv6.rs || true
cargo expand --all-features --test mod validation::length >> temp/length.rs || true
cargo expand --all-features --test mod validation::range >> temp/range.rs || true
cargo expand --all-features --test mod validation::prefix >> temp/prefix.rs || true
cargo expand --all-features --test mod validation::suffix >> temp/suffix.rs || true
cargo expand --all-features --test mod validation::url >> temp/url.rs || true
cargo expand --all-features --test mod validation::pattern >> temp/pattern.rs || true
cargo expand --all-features --test mod validation::option >> temp/option.rs || true

cargo check --all-features --test mod
