FROM rust:1.57-buster as build

ENV BASE /opt/cartesi
RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y cmake && \
    rm -rf /var/lib/apt/lists/*
RUN export ARCH=$(uname -m | sed 's/aarch64/aarch_64/') && \
   curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.20.1/protoc-3.20.1-linux-$ARCH.zip && \
   unzip protoc-3.20.1-linux-$ARCH.zip -d $HOME/.local
   
# install wagyu utility for mnemonic handling
RUN cargo install wagyu --locked

WORKDIR $BASE

COPY ./arbitration-dlib/ $BASE/arbitration-dlib
COPY ./logger-dlib/ $BASE/logger-dlib
COPY ./ipfs_interface/ $BASE/ipfs_interface

WORKDIR $BASE/cartesi_compute

# Compile cartesi_compute
COPY ./cartesi_compute/Cargo.toml ./
COPY ./cartesi_compute/Cargo.lock ./
COPY ./cartesi_compute/src ./src

RUN PATH="$PATH:$HOME/.local/bin" cargo install -j $(nproc) --locked --path .


# Onchain image to retrieve deployment info from NPM dependencies
FROM node:14-alpine as onchain

RUN apk add --no-cache \
    build-base \
    git \
    openssl \
    python3 \
    py3-pip

WORKDIR /opt/cartesi
COPY yarn.lock .
COPY package.json .

RUN yarn install --ignore-scripts
RUN yarn postinstall

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
COPY --from=build /usr/local/cargo/bin/cartesi_compute $BASE/bin/cartesi_compute
COPY --from=build /usr/local/cargo/bin/wagyu /usr/local/bin

# Copy dispatcher scripts
COPY ./dispatcher-entrypoint.sh $BASE/bin/
COPY ./config-template.yaml $BASE/etc/compute/
RUN mkdir -p $BASE/srv/compute

# Copy deployments info
COPY ./deployments $BASE/share/blockchain/deployments
COPY --from=onchain $BASE/node_modules/@cartesi/arbitration/deployments $BASE/share/blockchain/deployments

ENV ETHEREUM_HOST "hardhatnet"
ENV ETHEREUM_PORT "8545"
ENV ETHEREUM_TIMEOUT "120s"

ENTRYPOINT $BASE/bin/dispatcher-entrypoint.sh
