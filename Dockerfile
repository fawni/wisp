FROM rust:1.67 as builder
WORKDIR /usr/src/wisp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wisp /app/wisp
CMD ["/app/wisp"]