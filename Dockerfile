# Use official Rust image
FROM rust:latest AS builder

WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source
COPY . .

# Build the real binary
RUN cargo build --release

# Final slim image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rust-server-2024 /usr/local/bin/rust-server-2024

EXPOSE 3000

CMD ["rust-server-2024"]
