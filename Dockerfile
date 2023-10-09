FROM rust:1.72.0-slim-buster as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu


FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-gnu/release/maip /usr/local/bin/maip

WORKDIR /usr/local/bin

ENTRYPOINT ["maip"]