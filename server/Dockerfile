FROM rust:1.79
WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY src ./src

WORKDIR /app/src

RUN cargo build

RUN cargo install --path ../

RUN cargo install cargo-watch

EXPOSE 3001

CMD ["cargo", "watch", "-x", "run"]