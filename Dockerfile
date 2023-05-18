FROM rust:latest as build

RUN USER=root cargo new --bin embloy_feed_generator
WORKDIR /embloy_feed_generator

COPY src/Cargo.lock ./Cargo.lock
COPY src/Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release
RUN chmod +x /embloy_feed_generator/target/release/embloy_feed_generator

EXPOSE 8080

CMD ["/embloy_feed_generator/target/release/embloy_feed_generator"]

# CREATE IMAGE:
# docker build -t embloy_feed_generator .
