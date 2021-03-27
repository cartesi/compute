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
COPY ./ipfs_interface/ $BASE/ipfs_interface

WORKDIR $BASE/descartes

# Compile descartes
COPY ./descartes/Cargo.toml ./
COPY ./descartes/Cargo.lock ./
COPY ./descartes/src ./src

RUN cargo install -j $(nproc) --path .


# Onchain image to retrieve deployment info from NPM dependencies
FROM node:14-alpine as onchain

RUN apk add --no-cache \
    build-base \
    git \
    openssl \
    python \
    py-pip

WORKDIR /opt/cartesi
COPY yarn.lock .
COPY package.json .

RUN yarn install --ignore-scripts


# Runtime image
FROM debian:buster-slim as runtime

ENV BASE /opt/cartesi

RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y ca-certificates wget gettext jq curl && \
    rm -rf /var/lib/apt/lists/*

ENV DOCKERIZE_VERSION v0.6.1
RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && tar -C /usr/local/bin -xzvf dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
    && rm dockerize-linux-amd64-$DOCKERIZE_VERSION.tar.gz

WORKDIR /opt/cartesi

# Copy the build artifacts from the build stage
COPY --from=build /usr/local/cargo/bin/descartes $BASE/bin/descartes
COPY --from=build /usr/local/cargo/bin/wagyu /usr/local/bin

# Copy dispatcher scripts
COPY ./dispatcher-entrypoint.sh $BASE/bin/
COPY ./config-template.yaml $BASE/etc/descartes/
RUN mkdir -p $BASE/srv/descartes

# Copy deployments info
COPY ./deployments $BASE/share/blockchain/deployments
COPY --from=onchain $BASE/node_modules/@cartesi/arbitration/deployments $BASE/share/blockchain/deployments

ENV ETHEREUM_HOST "hardhatnet"
ENV ETHEREUM_PORT "8545"
ENV ETHEREUM_TIMEOUT "120s"

ENTRYPOINT $BASE/bin/dispatcher-entrypoint.sh
