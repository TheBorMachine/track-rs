FROM rust:slim-trixie AS builder

WORKDIR /usr/src/app

RUN rustup target add aarch64-unknown-linux-gnu

COPY Cargo.toml ./
COPY src src
RUN cargo build --target aarch64-unknown-linux-gnu --release

FROM debian:trixie-slim

COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-gnu/release/ustats /usr/local/bin/ustats

EXPOSE 8000

CMD ["ustats"]