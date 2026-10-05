#!/usr/bin/env just --justfile

set shell := ['bash', '-c']

@_default:
    just --list

# Run all tests
test *args:
    cargo test --all-targets {{args}}

# Reformat all code
fmt:
    cargo fmt --all

# Check formatting without changing anything
test-fmt:
    cargo fmt --all -- --check

# Lint with clippy
clippy *args:
    cargo clippy --all-targets --all-features -- -D warnings {{args}}

# Type-check without building
check:
    cargo check --all-targets --all-features

# Build the docs, fail on broken links
check-doc:
    RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features

# Open the docs
docs: check-doc
    cargo doc --no-deps --all-features --open

# Run benchmarks
bench *args:
    cargo bench {{args}}

# Format, check and lint
lint: fmt check clippy

# Everything CI runs
ci-test: test-fmt clippy check-doc test

# Print the minimum supported Rust version from Cargo.toml
get-msrv:
    @cargo metadata --no-deps --format-version 1 | jq -r '.packages[0].rust_version'

# Check the crate builds and tests on the MSRV toolchain
test-msrv:
    rustup toolchain install "$(just get-msrv)" --profile minimal
    cargo +"$(just get-msrv)" test --all-targets

# Delete build artifacts
clean:
    cargo clean
