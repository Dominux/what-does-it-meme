FROM rustlang/rust:nightly

WORKDIR /app

# Creating dummy main.rs file
RUN mkdir ./src
RUN echo "fn main(){}" > ./src/main.rs

# Copying deps and downloading and pre-building them
COPY ./server/Cargo.toml .
RUN cargo build --tests

# Copying all the logic
COPY ./server .
