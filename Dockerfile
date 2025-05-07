# Builder stage
FROM rust:1.84-slim as builder

# Install only necessary build dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    build-essential \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app


# Cache dependencies
# COPY Cargo.toml ./
# # RUN mkdir src && echo 'fn main() {}' > src/main.rs
# # RUN cargo build --release
# # RUN rm -rf src
# RUN mkdir -p src && \
#     echo 'fn main() {println!("Hello world");}' > src/main.rs && \
#     cargo build --release && \
#     rm -rf src

# copy all files for 
COPY . .
run rm -rf Cargo.lock

RUN cargo build --release

# FROM alpine:latest

# RUN apk add --no-cache \
#     ca-certificates \
#     libgcc \
#     libstdc++ \
#     libpq
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*


WORKDIR /app
COPY --from=builder /app/target/release/rest_utils ./api

EXPOSE 8282