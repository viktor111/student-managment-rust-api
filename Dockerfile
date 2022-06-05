# Rust as the base image
FROM rust:1.61.0 as build

# Create a new empty shell project
RUN USER=root cargo new --bin students-managment-system
WORKDIR /students-managment-system

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN cargo build --release

# The final base image
FROM debian:buster-slim

# Copy from the previous build
COPY --from=build /students-managment-system/target/release/students-managment-system /usr/src/students-managment-system
# COPY --from=build /students-managment-system/target/release/students-managment-system/target/x86_64-unknown-linux-musl/release/students-managment-system .

# Run the binary
CMD ["/usr/src/students-managment-system"]