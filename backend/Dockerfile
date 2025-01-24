# Use the official Rust image for building
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /app

# Copy the backend's Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Fetch dependencies to avoid fetching them again during the build
RUN cargo fetch

# Copy the rest of the application source code
COPY src ./src
COPY migrations ./migrations 

# Build the application in release mode
RUN cargo build --release

# Now use a minimal image to run the Rust application
FROM ubuntu:22.04

# Install necessary dependencies to run the Rust app
RUN apt-get update && apt-get install -y libpq-dev && apt-get clean

# Set the working directory inside the final container
WORKDIR /app

# Copy the compiled binary from the builder image
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

# Expose the port that the app will run on
EXPOSE 8000

# Run the application
CMD ["backend"]
