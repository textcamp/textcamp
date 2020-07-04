# builder

FROM rust:1.44.1-alpine as builder

WORKDIR /usr/src/myapp

COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release
RUN cargo install --path .

# target image

FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server

CMD ["server"]
