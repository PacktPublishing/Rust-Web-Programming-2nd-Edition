#!/bin/bash
FROM rust:1.61 as builder

# RUN apt-get update -yqq && apt-get install -yqq cmake g++
RUN apt-get update -yqq && apt-get install -yqq cmake g++

RUN cargo install diesel_cli --no-default-features --features postgres

# COPY ./src ./src
# COPY ./Cargo.toml ./Cargo.toml
# COPY ./target/release/web_app ./web_app
# COPY ./config.yml ./config.yml
COPY . .

WORKDIR .

# RUN rm -rf ./target/

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder ./target/release/web_app .
# COPY --from=builder ./web_app .
COPY --from=builder ./config.yml .
RUN chmod +x ./web_app

EXPOSE 8000

CMD ["./web_app", "config.yml"]
