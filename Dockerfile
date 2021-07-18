FROM rustlang/rust:nightly as builder
RUN apt-get update \
    && apt install -y ca-certificates libudev-dev --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new --bin rust-server
WORKDIR /rust-server

# Copy manifests
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy the source code
COPY src/ ./src/
COPY Rocket.toml Rocket.toml

# Build for release.
RUN rm -rf target/release/deps/rust-server*
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM debian:buster-slim as compiled
RUN apt-get update \
    && apt-get install -y ca-certificates libudev-dev --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /rust-server/target/x86_64-unknown-linux-musl/release/rust-server /bin/rust-server
RUN chmod +x /bin/rust-server

WORKDIR tmp
COPY Rocket.toml Rocket.toml

EXPOSE 7373

ENTRYPOINT ["/bin/rust-server"]