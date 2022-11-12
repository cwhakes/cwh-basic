FROM rust:1.65 as builder

# Make a dummy
WORKDIR /usr/src
RUN USER=root cargo new cwh-basic
COPY Cargo.toml Cargo.lock /usr/src/cwh-basic/
WORKDIR /usr/src/cwh-basic
RUN cargo build --release

# Make the real thing
COPY src /usr/src/cwh-basic/src/
RUN touch src/main.rs
RUN cargo build --release \
    && mv target/release/cwh-basic /bin

FROM debian:buster-slim as runner
WORKDIR /app
COPY --from=builder /bin/cwh-basic /bin/cwh-basic
COPY . /app

CMD ROCKET_DATABASES="{content_db={url=${DATABASE_URL}, pool_size=8}}" \
    ROCKET_PORT=$PORT cwh-basic
