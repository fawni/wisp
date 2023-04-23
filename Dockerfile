FROM rust:1.67-alpine AS builder
WORKDIR /usr/src/wisp
COPY . .
# hadolint ignore=DL3018
RUN apk add --no-cache libressl-dev musl-dev
# hadolint ignore=DL3059
RUN cargo install --path .

FROM alpine:3.14
# hadolint ignore=DL3018
RUN apk add --no-cache libressl-dev pkgconfig
WORKDIR /app
COPY --from=builder /usr/local/cargo/bin/wisp /app/wisp
CMD ["/app/wisp"]