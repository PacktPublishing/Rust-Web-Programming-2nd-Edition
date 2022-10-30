FROM rust:1.62.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app
COPY . .

ARG ENV="PRODUCTION"
RUN echo "$ENV"

RUN if [ "$ENV" = "PRODUCTION" ] ; then cargo build --release ; else cargo build ; fi
RUN if [ "$ENV" = "PRODUCTION" ] ; then echo "no need to copy" ; else mkdir /app/target/release/ && cp /app/target/debug/task_queue /app/target/release/task_queue ; fi

FROM gcr.io/distroless/cc-debian10

COPY --from=build /app/target/release/task_queue /usr/local/bin/task_queue

EXPOSE 3000
ENTRYPOINT ["task_queue"]
