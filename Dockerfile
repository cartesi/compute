FROM rust:1.44 as build

ENV BASE /opt/cartesi
RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y cmake protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# install wagyu utility for mnemonic handling
RUN cargo install wagyu

WORKDIR $BASE/descartes

# Compile dependencies
COPY ./descartes/Cargo_cache.toml ./Cargo.toml
RUN mkdir -p ./src && echo "fn main() { }" > ./src/main.rs
RUN cargo build -j $(nproc) --release

WORKDIR $BASE

COPY ./arbitration-dlib/ $BASE/arbitration-dlib
COPY ./logger-dlib/ $BASE/logger-dlib

WORKDIR $BASE/descartes

# Compile descartes
COPY ./descartes/Cargo.toml ./
COPY ./descartes/Cargo.lock ./
COPY ./descartes/src ./src

RUN cargo install -j $(nproc) --path .

# Runtime image
FROM debian:buster-slim as runtime

ENV BASE /opt/cartesi

RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates wget gettext jq && \
    rm -rf /var/lib/apt/lists/*

ENV DOCKERIZE_VERSION v0.6.1

ARG ARCH

ENV ARCH_E ${ARCH}

RUN if [ "$ARCH_E" = "ARM" ] ; then wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-armhf-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-linux-armhf-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-linux-armhf-$DOCKERIZE_VERSION.tar.gz \
	; else wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz ; fi

WORKDIR /opt/cartesi

# Copy the build artifacts from the build stage
COPY --from=build /usr/local/cargo/bin/descartes $BASE/bin/descartes
COPY --from=build /usr/local/cargo/bin/wagyu /usr/local/bin

# Copy dispatcher scripts
COPY ./dispatcher-entrypoint.sh $BASE/bin/
COPY ./config-template.yaml $BASE/etc/descartes/
RUN mkdir -p $BASE/srv/descartes

ENV ETHEREUM_HOST "ganache"
ENV ETHEREUM_PORT "8545"
ENV ETHEREUM_TIMEOUT "120s"

ENTRYPOINT $BASE/bin/dispatcher-entrypoint.sh
