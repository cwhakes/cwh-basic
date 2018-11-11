FROM centos:7
MAINTAINER Will Hakes <info@cwilliamhakes.com>

ENV SOURCES=/sources

RUN yum update -y
RUN yum install -y file gcc openssl-devel
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --disable-sudo

RUN mkdir -p $SOURCES
COPY ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release

CMD ROCKET_ENV=production ROCKET_PORT=$PORT ./target/release/cwh_helloworld
