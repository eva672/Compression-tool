# Build stage
FROM rust:1.75-slim as builder

WORKDIR /app

COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-compressor /app/rust-compressor

ENTRYPOINT ["/app/rust-compressor"] 