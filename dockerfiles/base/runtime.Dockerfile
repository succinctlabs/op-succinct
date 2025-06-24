# syntax=docker/dockerfile:1.4
# Minimal runtime base image for OP Succinct binaries
FROM debian:bookworm-slim AS runtime-base

# Install only essential runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get autoremove -y \
    && apt-get clean

# Create app directory
WORKDIR /app

# Create non-root user for security
RUN useradd --create-home --shell /bin/bash --user-group --uid 1000 opuser

# Set default user
USER opuser