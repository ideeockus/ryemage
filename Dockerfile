# stage 1: build
FROM rust:latest AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./image_processing/ ./image_processing/
COPY ./tg_controller/ ./tg_controller/
COPY ./src/ ./src/
RUN cargo build --release

# stage 2: release container
FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/tg_controller .

CMD ["./tg_controller"]
