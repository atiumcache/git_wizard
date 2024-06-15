FROM rust:latest

WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files first
COPY Cargo.toml Cargo.lock ./

COPY src ./src
# This will build the dependencies and cache them, which speeds up subsequent builds
RUN cargo build --release


# Build the final release version of your application
RUN cargo install --path .

# Set the entrypoint to your binary
ENTRYPOINT ["git_wizard"]
