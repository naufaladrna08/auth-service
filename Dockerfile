# Use an official Rust image as the builder
FROM rust:1.81 AS builder

# Install build dependencies and set up the workspace
RUN apt-get update && apt-get install -y \
  protobuf-compiler \
  libpq-dev \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the project files
COPY . .

# Build the Rust project
RUN cargo build --release

# Final image with only runtime dependencies
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
  libc6 libprotobuf-dev libpq5 \
  protobuf-compiler curl ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/app

# Copy only the compiled binary from the builder
COPY --from=builder /usr/src/app/target/release/auth /usr/bin/auth_service

# Expose the gRPC port
EXPOSE 50002

# Set the entrypoint
ENTRYPOINT ["auth_service"]
