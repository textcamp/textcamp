FROM rust:1.44.1-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN apk add --no-cache musl-dev
RUN cargo install --path .

# App configuration ------------------

# What log level the container operates at
ENV RUST_LOG=info

# The URL (host and port) the server runs on
ENV SERVER_URL="0.0.0.0:8080"

# The path to the templates
ENV WORLD_ROOT="./world"

CMD ["server"]