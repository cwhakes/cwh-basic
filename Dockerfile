FROM rust:1.65 as builder
ARG NAME=cwh-basic

# Make a dummy
WORKDIR /usr/src
RUN USER=root cargo new $NAME
WORKDIR /usr/src/$NAME
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Make the real thing
COPY src src/
RUN touch src/main.rs \
    && cargo build --release \
    && mv target/release/$NAME /bin

FROM debian:buster-slim as runner
ARG NAME=cwh-basic

WORKDIR /app
COPY static static/
COPY templates templates/
COPY Rocket.toml ./
COPY --from=builder /bin/$NAME /bin/

ENV NAME=$NAME
CMD ROCKET_PORT=$PORT \
    cwh-basic
