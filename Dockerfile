FROM rust:1.44.1-alpine as build

WORKDIR /usr/src/textcamp

COPY . .

RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM alpine:latest

COPY --from=build /usr/src/textcamp/site /usr/textcamp/site
COPY --from=build /usr/src/textcamp/world /usr/textcamp/world
COPY --from=build /usr/src/textcamp/.env /usr/textcamp/.env
COPY --from=build /usr/src/textcamp/target/release/server /usr/textcamp/server

WORKDIR /usr/textcamp

ENV PATH=/usr/textcamp:$PATH
ENV RUST_LOG=info

CMD ["./server"]