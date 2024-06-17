# Build stage
FROM rust:1.76 as builder

COPY ./src ./src
COPY ./assets ./assets
COPY ./assets/css ./assets/css
COPY ./templates ./templates
COPY Cargo.toml ./

RUN cargo build --release

# Prod stage
FROM debian:stable-slim

EXPOSE 8080

COPY --from=builder /target/release/hteamx /

ENTRYPOINT ["./hteamx"]
