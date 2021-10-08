FROM rust:1-slim-stretch AS builder
RUN rustup install stable-x86_64-unknown-linux-gnu
RUN rustup default stable

RUN apt-get update && apt-get install -y libclang-dev

COPY Cargo.lock Cargo.toml /sources/
COPY ./src/ /sources/src/

WORKDIR /sources
RUN cargo build --release
RUN chown nobody:nogroup /sources/target/release/faas


FROM debian:stretch-slim
COPY fortunes /opt/fortunes
COPY --from=builder /sources/target/release/faas /opt/faas

USER nobody
EXPOSE 8000
WORKDIR /opt
ENTRYPOINT ["/opt/faas"]
