FROM rust:1.65 AS builder

LABEL authors="EpictetusZ1"

# Set the working directory in the Docker image
WORKDIR /usr/src/rust-api

# Copy the source code of your Rust API into the Docker image
COPY ./apps/rust-api/Cargo.lock ./Cargo.lock
COPY ./apps/rust-api/Cargo.toml ./Cargo.toml

# Create a dummy main file to build deps
RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/keywords*

# Copy the actual source code into the Docker image
COPY ./apps/rust-api/src ./src

# Build your Rust application for release
RUN cargo build --release
#RUN --mount=type=cache,target=/usr/local/cargo/registry \
 #    cargo build --release

# Stage 2: Setting up the runtime environment
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage to the runtime stage
COPY --from=builder /usr/src/rust-api/target/release/keywords /usr/local/bin/keywords

# Command to run the application
CMD ["keywords"]
