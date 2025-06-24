# syntax=docker/dockerfile:1.4
# Shared base image with Rust + SP1 for all OP Succinct builds
FROM rust:1.85 AS rust-sp1-base

# Install build dependencies in a single layer
RUN apt-get update && apt-get install -y \
    build-essential \
    libclang-dev \
    llvm-dev \
    pkg-config \
    libssl-dev \
    git \
    protobuf-compiler \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install SP1 in a separate layer for better caching
RUN curl -L https://sp1.succinct.xyz | bash && \
    ~/.sp1/bin/sp1up && \
    ~/.sp1/bin/cargo-prove prove --version

# Set working directory
WORKDIR /build

# Create cache mount points
VOLUME ["/root/.cargo/registry", "/build/target"]