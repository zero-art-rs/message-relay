# syntax=docker/dockerfile:1
FROM rust:1.85-alpine3.20 as builder
ENV RUSTFLAGS="-C target-feature=-crt-static"

RUN apk add --no-cache musl-dev openssl-dev build-base git openssh-client protobuf protobuf-dev

WORKDIR /opt

COPY Cargo.lock .
COPY Cargo.toml .

RUN cargo build --release || true

COPY src src/

RUN cargo build --release -p message-relay \
	&& mkdir out \
	&& cp target/release/message-relay out/ \
	&& strip out/message-relay

FROM alpine:3.20

RUN apk add --no-cache libgcc openssl postgresql-client

COPY --from=builder /opt/out/message-relay /bin/message-relay

CMD ["/bin/message-relay", "run", "--config", "/config.toml"]