FROM rust:latest AS builder
WORKDIR /app
COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp /app/target/release/tmd-aws-exporter /app/tmd-aws-exporter

FROM debian:latest
WORKDIR /app
COPY --from=builder /app/tmd-aws-exporter /app
RUN apt update && apt install -y ca-certificates
LABEL org.opencontainers.image.source="https://github.com/korewaChino/tmd-aws-exporter"

CMD ["/app/tmd-aws-exporter"]
