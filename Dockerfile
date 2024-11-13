from rust:1.74 as builder
workdir /usr/src/wisp
copy . .
run cargo build --release && mv ./target/release/wisp ./wisp

from debian:bookworm-slim
# hadolint ignore=DL3008
run apt-get update && apt-get install -y --no-install-recommends libssl-dev pkg-config ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/*
workdir /app
copy --from=builder /usr/src/wisp/wisp /app/wisp
cmd ["/app/wisp"]
