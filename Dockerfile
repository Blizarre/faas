FROM dhi.io/rust:1-alpine3.22-dev AS builder

COPY . /sources
WORKDIR /sources
RUN cargo build --release

FROM dhi.io/rust:1-alpine3.22
COPY --from=builder /sources/target/release/faas /opt/faas

WORKDIR /etc/secrets

EXPOSE 8000
ENTRYPOINT ["/opt/faas"]
