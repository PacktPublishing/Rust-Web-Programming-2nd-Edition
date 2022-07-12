FROM rust:1.61

RUN apt-get update -yqq && apt-get install -yqq cmake g++
RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

WORKDIR .

RUN cargo clean
RUN cargo build --release
RUN cp ./target/release/web_app ./web_app
RUN rm -rf ./target 
RUN rm -rf ./src
RUN rm config.yml
RUN chmod +x ./web_app  

EXPOSE 8000

CMD ["./web_app", "config.yml"]
