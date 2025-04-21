# stage 1: build
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./image_processing/ ./image_processing/
COPY ./tg_controller/ ./tg_controller/
RUN cargo build --release

# stage 2: release container
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/ryemage_bot .

CMD ["./ryemage_bot"]
