FROM rust:latest

RUN curl -sSL https://github.com/foundry-rs/foundry/releases/download/nightly/foundry_nightly_linux_$(dpkg --print-architecture).tar.gz | \
    tar -zx -C /usr/local/bin

# Install protoc
WORKDIR /usr
RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v24.4/protoc-24.4-linux-x86_64.zip \
        && unzip protoc-24.4-linux-x86_64.zip \
        && rm protoc-24.4-linux-x86_64.zip

# Copy source
#ADD . /source
WORKDIR /source