FROM rust:1.50-buster as builder

RUN USER=root cargo new --bin people-logger
WORKDIR ./people-logger
COPY ./Cargo.toml ./Cargo.toml
RUN rustup override set nightly
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/people_logger*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt update \
    && apt install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /people-logger/target/release/people-logger ${APP}/people-logger

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./people-logger"]
