FROM rust:1.87

WORKDIR /app

COPY ./Cargo.toml ./Cargo.lock ./
COPY . .

RUN cargo build --release

CMD ["./target/release/server"]
