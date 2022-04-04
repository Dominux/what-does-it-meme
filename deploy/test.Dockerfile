FROM rust:1.59

WORKDIR /app

COPY ./Cargo.toml .
COPY ./Cargo.lock .

RUN cargo install --path .

COPY . .

RUN cargo build
