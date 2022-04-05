FROM rust:1.59

WORKDIR /app

COPY ./server .

RUN cargo install --path .
RUN cargo build
