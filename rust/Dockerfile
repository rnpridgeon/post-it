FROM rust:1-slim-buster AS builder

WORKDIR /usr/src/server
COPY . .

RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir -p /app

COPY --from=builder /usr/src/server/target/release/post_it /app/server

CMD /app/server