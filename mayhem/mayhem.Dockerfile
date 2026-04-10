# Build Stage
FROM --platform=linux/amd64 ubuntu:22.04 AS BUILDER

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install -f cargo-fuzz

ADD . /src
WORKDIR /src

RUN cargo +nightly fuzz build

# Package Stage
FROM ubuntu:22.04 AS PACKAGE

COPY --from=BUILDER /src/fuzz/target/x86_64-unknown-linux-gnu/release/fuzz_* /fuzzers/
