FROM rust:1.57 as builder

RUN USER=root cargo new --bin rust-docker-grpc
WORKDIR /rust-docker-grpc
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm -rf ./target/release/deps/rust_docker_grpc*
RUN rustup component add rustfmt
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 50051

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-docker-grpc/target/release/aeonai_calcs ${APP}/aeonai_calcs

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./aeonai_calcs"]