FROM rust:alpine AS builder

WORKDIR /build
COPY . .

RUN apk add --no-cache musl-dev openssl-dev
RUN rustup default nightly
RUN cargo build --release

FROM alpine:latest

WORKDIR /build
ADD schema schema
COPY --from=builder \
    /build/target/release/database-enforcer \
    /build

ENV DISCORD_TOKEN ""
ENTRYPOINT ./database-enforcer
