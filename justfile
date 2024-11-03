alias b := build
alias e := example
alias f := format
alias t := test
alias c := check
alias d := develop
alias du := docker_up
alias dd := docker_down

# COMMANDS -----------------------------------------------------------------------------------------

# List commands
default:
    @just --list

# Check
check:
    cargo check --all

# Format
format: check
    cargo +nightly fmt

# Test
test: format
    cargo test --all

# Build
build: test
    cargo build --release

# Run an example
example EXAMPLE=("basic"): docker_up && docker_down
    -cargo run --package lib_frankfurter --example {{ EXAMPLE }}

# Run test suite whenever any change is made
develop:
    cargo watch -w cli -w lib -s "just test"

# Start up the local Frankfurter API
docker_up:
    docker compose up -d --wait

# Shut down the local Frankfurter API
docker_down:
    docker compose down
