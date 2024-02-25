FROM rust:latest
LABEL authors="yellowhatpro"

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=6666

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["./target/release/server"]
