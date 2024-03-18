set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Install
install:
    cargo install --path .

# Runs exessive clippy lints (possible false positives so just warn)
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery

# Deploy to fly.io
deploy:
    fly deploy --remote-only

push:
    git push
    git push gh

update: && install
    git pull