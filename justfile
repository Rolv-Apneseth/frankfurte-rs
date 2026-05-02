alias b := build
alias e := example
alias f := format
alias t := test
alias c := check
alias d := develop
alias du := podman_up
alias dd := podman_down
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
test: podman_up && podman_down
    cargo test --all -- --nocapture

# Run test suite whenever any change is made
develop: format podman_up
    bacon test --all-features

# Build
build: format
    cargo build --release

# Run an example
example EXAMPLE=("basic"): podman_up && podman_down
    -cargo run --package lib_frankfurter --example {{ EXAMPLE }}

# Start up the local Frankfurter API
podman_up:
    podman compose up -d --wait

# Shut down the local Frankfurter API
podman_down:
    podman compose down

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
