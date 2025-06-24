# Build stage - use shared base image
FROM rust:1.85 AS rust-sp1-base

# Install build dependencies in a single layer
RUN apt-get update && apt-get install -y \
    build-essential \
    libclang-dev \
    llvm-dev \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install SP1 in a separate layer for better caching
RUN curl -L https://sp1.succinct.xyz | bash && \
    ~/.sp1/bin/sp1up && \
    ~/.sp1/bin/cargo-prove prove --version

# Dependency caching stage
FROM rust-sp1-base AS dependencies

WORKDIR /build

# Copy dependency manifests first for better caching
COPY Cargo.toml Cargo.lock ./
COPY utils/*/Cargo.toml ./utils/*/
COPY programs/*/Cargo.toml ./programs/*/
COPY programs/aggregation/Cargo.toml ./programs/aggregation/
COPY programs/range/utils/Cargo.toml ./programs/range/utils/
COPY programs/range/ethereum/Cargo.toml ./programs/range/ethereum/
COPY programs/range/celestia/Cargo.toml ./programs/range/celestia/
COPY validity/Cargo.toml ./validity/
COPY fault-proof/Cargo.toml ./fault-proof/

# Create stub source files to satisfy cargo build
RUN find . -name "Cargo.toml" -exec dirname {} \; | xargs -I {} mkdir -p {}/src && \
    find . -name "Cargo.toml" -exec dirname {} \; | xargs -I {} touch {}/src/lib.rs

# Build dependencies only (this layer will be cached)
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --bin validity --release

# Builder stage
FROM dependencies AS builder

# Copy actual source code
COPY . .

# Build the server (only source changes trigger rebuild)
RUN --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --bin validity --release && \
    cp target/release/validity /build/validity-proposer

# Final stage - minimal runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install only essential runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    jq \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get autoremove -y \
    && apt-get clean

# Create non-root user for security (check if UID 1000 exists first)
RUN if ! id 1000 >/dev/null 2>&1; then \
        useradd --create-home --shell /bin/bash --user-group --uid 1000 opuser; \
    else \
        useradd --create-home --shell /bin/bash --user-group opuser; \
    fi

# Copy only the built binary from builder
COPY --from=builder /build/validity-proposer /usr/local/bin/validity-proposer

# Copy only necessary SP1 artifacts (not the full installation)
COPY --from=builder /root/.sp1/circuits/ /app/circuits/

# Set proper permissions
RUN chown -R opuser:opuser /app && \
    chmod +x /usr/local/bin/validity-proposer

# Switch to non-root user
USER opuser

# Run the server from its permanent location
CMD ["/usr/local/bin/validity-proposer"]
