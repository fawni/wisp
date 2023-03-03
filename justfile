set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build
build:
    cargo build --locked --release

# Runs exessive clippy lints (possible false positives so just warn)
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery