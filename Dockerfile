FROM centos:7
MAINTAINER Will Hakes <info@cwilliamhakes.com>

ENV SOURCES=/sources
ENV PATH="/root/.cargo/bin:${PATH}"

RUN yum update -y
RUN yum install -y file gcc openssl-devel
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y -v --default-toolchain nightly-2020-01-02

RUN mkdir -p $SOURCES
COPY ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release

CMD ROCKET_PORT=$PORT ./target/release/cwh-basic