FROM clux/muslrust:stable as builder
USER root
WORKDIR /app

RUN apt-get update && \
    apt-get upgrade -y

RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY ./src ./src
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3 AS runtime

RUN apk update && \
    apk upgrade

RUN addgroup -S simple-api-user && \
    adduser -S simple-api-user -G simple-api-user

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/simple-api /app/simple-api

USER simple-api-user

CMD ["/app/simple-api"]
