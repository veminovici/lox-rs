#!/bin/bash

echo "Generating code coverage..."

# Install the tool
echo "Installing grcov ..."
cargo install grcov

# Enable the required Rust flags to generate the coverage files
echo "Set the required rust flags..."
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"

# grcov requires rust nightly at this time
echo "Running the test using nightly rust.."
cargo +nightly test

# generate the html report
echo "Generating the coverage report..."
grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/

# open the report
open target/debug/coverage/index.html
