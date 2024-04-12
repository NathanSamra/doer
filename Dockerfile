FROM rust:1.77.2
WORKDIR /app
COPY ./Cargo.toml ./
COPY ./Cargo.lock ./
COPY ./src ./src
