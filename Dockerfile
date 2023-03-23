FROM rust:slim AS builder

# hadolint ignore=DL3008
RUN apt-get update && apt-get -y --no-install-recommends install libssl-dev pkg-config

WORKDIR /usr/src/wisp
COPY . .

RUN cargo fetch

# hadolint ignore=DL3059
RUN cargo install --path .

FROM rust:slim

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/wisp /app/wisp
CMD ["/app/wisp"]