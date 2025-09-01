alias b := build
alias e := example
alias f := format
alias t := test
alias c := check
alias d := develop
alias du := docker_up
alias dd := docker_down
alias p := publish
alias g := gif
alias dg := develop-gif

# List commands
default:
    @just --list

# Check
check:
    cargo check --all

# Format and lint
format:
    cargo +nightly fmt
    cargo clippy --all -- -D warnings 

# Test
test: docker_up && docker_down
    cargo test --all -- --nocapture

# Run test suite whenever any change is made
develop: format docker_up
    cargo watch -w cli -w lib -s "cargo test --all -- --nocapture"

# Build
build: format
    cargo build --release

# Run an example
example EXAMPLE=("basic"): docker_up && docker_down
    -cargo run --package lib_frankfurter --example {{ EXAMPLE }}

# Start up the local Frankfurter API
docker_up:
    docker compose up -d --wait

# Shut down the local Frankfurter API
docker_down:
    docker compose down

# Publish both the binary and library crates to crates.io
publish: test
    cargo publish --package lib_frankfurter
    cargo publish --package frankfurter_cli

# Generate the demo GIF
gif:
    vhs assets/demo.tape --output assets/demo.gif

# Re-generate the demo GIF whenever `demo.tape` is modified
develop-gif:
    echo assets/demo.tape | entr vhs /_ --output assets/demo.gif
