FROM rust:latest as build

RUN USER=root cargo new --bin Embloy_feed_generator
WORKDIR /Embloy_feed_generator

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN cargo build --release
RUN chmod +x /Embloy_feed_generator/target/release/Embloy_feed_generator

EXPOSE 8081

CMD ["/Embloy_feed_generator/target/release/Embloy_feed_generator"]