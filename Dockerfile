FROM rust:alpine3.18 AS builder
WORKDIR /workdir
COPY ./ ./
RUN apk add --update --no-cache musl-dev pkgconfig libressl-dev &&\
    cargo build --release

FROM scratch
COPY --from=builder /workdir/target/release/omni-notify /
ENTRYPOINT ["/omni-notify"]
