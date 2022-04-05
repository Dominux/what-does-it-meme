FROM rust:1.59

WORKDIR /app

COPY ./server/Cargo.toml .
COPY ./server/Cargo.lock .

RUN mkdir ./src
COPY ./server/src/main.rs ./src/

RUN cargo install --path .

COPY ./server .

RUN cargo build
