FROM rust:latest AS builder
# ARG APP_NAME=rust-server-2024
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM rust:latest-alpine
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
