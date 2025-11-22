ARG APP_NAME=serenity_template

FROM rust:slim-bullseye AS builder
ARG APP_NAME

WORKDIR /app
COPY . .

RUN apt-get update && \
    apt-get install -y musl-tools upx && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl --bin  && \
    strip target/x86_64-unknown-linux-musl/release/ && \
    upx --best --lzma target/x86_64-unknown-linux-musl/release/

FROM gcr.io/distroless/static
ARG APP_NAME

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ /usr/local/bin/app

ENTRYPOINT ["/usr/local/bin/app"]
