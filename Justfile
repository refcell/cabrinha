set positional-arguments
alias t := tests
alias l := lints
alias f := fmt
alias b := build

# default recipe to display help information
default:
  @just --list

# Run all tests
tests: test test-docs

# Test for the native target with all features
test *args='':
  cargo nextest run --workspace --all --all-features $@

# Test the Rust documentation
test-docs:
  cargo test --doc --all --locked

# Lint the workspace for all available targets
lints: lint lint-docs

# Lint the workspace
lint: fmt-native-check
  cargo +nightly clippy --workspace --all --all-features --all-targets -- -D warnings

# Lint the Rust documentation
lint-docs:
  RUSTDOCFLAGS="-D warnings" cargo doc --all --no-deps --document-private-items

# Fixes the formatting of the workspace
fmt:
  cargo +nightly fmt --all

# Check the formatting of the workspace
fmt-native-check:
  cargo +nightly fmt --all -- --check

# Build
build *args='':
  cargo build --workspace --all $@
