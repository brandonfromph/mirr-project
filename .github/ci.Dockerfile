FROM ubuntu:24.04

# Install base dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    iverilog \
    python3 \
    npm \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Add components
RUN rustup component add clippy rustfmt

# Install wasm-pack
RUN cargo install wasm-pack --locked

# Install nextest
RUN cargo install cargo-nextest --locked

# Note: Rocq prover is currently managed via docker container in CI,
# so we don't install it in this base image to keep it lightweight.
# We will pull the rocq/rocq-prover:9.0 image in the specific job.
