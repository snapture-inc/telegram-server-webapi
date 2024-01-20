FROM rust:slim-buster AS BUILD_IMAGE

# Build error for OpenSSL: https://docs.rs/openssl/latest/openssl/
RUN apt-get update -y && \
  apt-get install -y pkg-config libssl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo build --release

FROM rust:slim-buster AS PRODUCTION_STAGE
WORKDIR /app
COPY --from=BUILD_IMAGE /app/target/release/telegram-server-webapi .
EXPOSE 8000
CMD ["./telegram-server-webapi"]