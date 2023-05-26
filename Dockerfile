FROM rust:1.67 as builder
WORKDIR /usr/src/wisp
COPY . .
RUN cargo build --release && mv ./target/release/wisp ./wisp

FROM debian:bullseye-slim
# hadolint ignore=DL3008
RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/wisp/wisp /app/wisp
CMD ["/app/wisp"]