# Use an official Rust image as the builder
FROM rust:1.81 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
  protobuf-compiler \
  libpq-dev \
  && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Build the Rust project
RUN cargo build --release

# FROM debian:bookworm-slim
FROM ubuntu:22.04

# Set the working directory
WORKDIR /usr/src/app

# Install the necessary dependencies for your application
RUN apt-get update && apt-get install -y \
  libc6 \
  libprotobuf-dev \
  libpq5 \
  protobuf-compiler \
  curl \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

RUN curl -sSL "https://github.com/fullstorydev/grpcurl/releases/download/v1.8.6/grpcurl_1.8.6_linux_x86_64.tar.gz" | tar -xz -C /usr/local/bin

# Copy the compiled binary from the builder
COPY --from=builder /usr/src/app/target/release/user /usr/bin/user_service

# Expose the gRPC port
EXPOSE 50001

# Set the entrypoint
ENTRYPOINT ["user_service"]
