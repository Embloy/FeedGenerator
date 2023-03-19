FROM rust:latest as build

RUN USER=root cargo new --bin embloy_feed_generator
WORKDIR /embloy_feed_generator

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/embloy_feed_generator*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /embloy_feed_generator/target/release/embloy_feed_generator .
CMD ["./embloy_feed_generator"]