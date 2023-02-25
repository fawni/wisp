set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build
build:
    cargo build --locked --release

# Runs clippy
check:
    cargo clippy --locked -- -D warnings

# Cut a release
@release VERSION:
    cargo release {{VERSION}} -x --no-publish --no-confirm