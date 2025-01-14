FROM rust:1.75.0-bookworm AS build

# apt
RUN apt update && apt install -y musl-dev musl-tools build-essential cmake

RUN mkdir /app
WORKDIR /app

# build cargo dependencies
COPY Cargo.toml Cargo.lock /app/
RUN rustup target add x86_64-unknown-linux-musl
RUN mkdir /app/src && \
    echo 'fn main() {}' > /app/src/main.rs && \
    cargo build --release --target x86_64-unknown-linux-musl && \
    rm -Rvf /app/src

# build app
COPY src /app/src/
RUN rm -rf /app/target/x86_64-unknown-linux-musl/release/.fingerprint/svci*
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.19.0 AS runtime

# apk
RUN apk add git
COPY --from=build /app/target/x86_64-unknown-linux-musl/release/svci /usr/local/bin/svci

FROM runtime as action

WORKDIR /app
ENTRYPOINT ["svci"]
