# stage 1: build
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./image_processing/ ./image_processing/
COPY ./tg_controller/ ./tg_controller/

RUN mkdir src && echo "fn main() {}" > src/main.rs

RUN cargo fetch
RUN cargo build --release

COPY . .

RUN cargo build --release

# stage 2: release container
FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/ryemage_bot .

CMD ["./ryemage_bot"]
