# syntax=docker/dockerfile:1

FROM rust:latest AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/lib.rs ./src/
RUN cargo fetch
COPY . .
RUN --mount=type=secret,id=CBAT_KEY_NAME,env=CBAT_KEY_NAME \
    --mount=type=secret,id=CBAT_KEY_SECRET,env=CBAT_KEY_SECRET \
    cargo test --release
RUN cargo build --release
#FROM debian:bookworm-slim as runtime
#WORKDIR /app
#COPY --from=builder /app/target/release/cbat .
#CMD ["./cbat"]