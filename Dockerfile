FROM rust:1.88.0-slim-trixie AS builder

WORKDIR /app

RUN cargo init --bin
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src .
