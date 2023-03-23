FROM rust:1.67 AS builder

# hadolint ignore=DL3008
RUN apt-get update && apt-get -y --no-install-recommends install libssl-dev pkg-config

RUN cargo install sccache

ENV HOME=/home/root
ENV SCCACHE_CACHE_SIZE="1G"
ENV SCCACHE_DIR=$HOME/.cache/sccache
ENV RUSTC_WRAPPER="/usr/local/cargo/bin/sccache"

WORKDIR /usr/src/wisp
COPY . .

RUN cargo fetch

# hadolint ignore=DL3059
RUN cargo install --path .

FROM rust:1.67

WORKDIR /app

COPY --from=builder /usr/local/cargo/bin/wisp /app/wisp
CMD ["/app/wisp"]