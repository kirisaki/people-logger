FROM rust:1.50-buster

RUN USER=root cargo new --bin people-logger
WORKDIR ./people-logger
COPY ./Cargo.toml ./Cargo.toml
RUN apt update && apt install -y postgresql-client
RUN rustup override set nightly
RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/people_logger*
RUN cargo build --release

RUN cp ./target/release/people-logger /usr/bin/
CMD ["people-logger"]

