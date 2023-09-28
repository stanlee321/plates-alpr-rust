# Build Stage
# FROM rust:1.55.0-slim-buster as builder
FROM rust:1.67.0 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# Runtime Stage
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/webhook_server .
ENV RUST_LOG=info
CMD ["./webhook_server"]
