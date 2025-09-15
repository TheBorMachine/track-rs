FROM rust:latest AS builder

WORKDIR /usr/src/app

RUN apt-get update && \ 
    apt-get install -y musl-tools musl-dev build-essential && \
    yes | apt-get install gcc-x86-64-linux-gnu && \
    rustup target add x86_64-unknown-linux-musl

ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'

COPY Cargo.toml ./
COPY src src
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/ustats /usr/local/bin/app

CMD ["app"]