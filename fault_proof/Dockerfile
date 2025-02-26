# syntax=docker/dockerfile:1.4

# Build stage
FROM ubuntu:24.04 AS builder

WORKDIR /usr/src/app

# Install Rust and required dependencies
RUN apt-get update && apt-get install -y \
    curl \
    clang \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly

# Verify Rust installation
RUN rustc --version && cargo --version

# Install SP1
RUN curl -L https://sp1.succinct.xyz | bash && \
    ~/.sp1/bin/sp1up && \
    ~/.sp1/bin/cargo-prove prove --version

# Copy the entire workspace
COPY . .

# Build the binaries
RUN cargo build --release --bin proposer --bin challenger

# Runtime stage
FROM ubuntu:24.04

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    curl \
    clang \
    build-essential \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH=/root/.cargo/bin:$PATH
RUN rustup install nightly
RUN rustup default nightly

# Install SP1
RUN curl -L https://sp1.succinct.xyz | bash && \
    ~/.sp1/bin/sp1up && \
    ~/.sp1/bin/cargo-prove prove --version

# Copy the binaries from builder
COPY --from=builder /usr/src/app/target/release/proposer /usr/local/bin/
COPY --from=builder /usr/src/app/target/release/challenger /usr/local/bin/

# Create directories for data and logs
RUN mkdir -p /app/data /app/logs/proposer /app/logs/challenger

# Set the entrypoint script
COPY fault_proof/docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["docker-entrypoint.sh"]
