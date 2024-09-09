# Use an official Rust image as a builder
FROM rust:latest AS builder

# Install musl-tools for static linking
RUN apt-get update && apt-get install -y musl-tools

# Set the work directory inside the container
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the binary for the x86_64-unknown-linux-musl target
RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target=x86_64-unknown-linux-musl

# Create a second, smaller image to run the compiled binary
FROM scratch

# Set the work directory inside the container
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/nitro_enclave_attestation /app/nitro_enclave_attestation

# Set the entrypoint to the binary
ENTRYPOINT ["/app/nitro_enclave_attestation"]

