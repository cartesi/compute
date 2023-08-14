FROM --platform=linux/amd64 cartesi/machine-emulator:0.14.0
RUN apt-get -y update; apt-get -y install curl
RUN curl -sSL https://github.com/foundry-rs/foundry/releases/download/nightly/foundry_nightly_linux_$(dpkg --print-architecture).tar.gz | \
    tar -zx -C /usr/local/bin

RUN apt-get install -y \
    build-essential \
    curl

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"
RUN \
    apt-get update && \
    apt-get install --no-install-recommends -y cmake unzip && \
    rm -rf /var/lib/apt/lists/*

RUN export ARCH=$(uname -m | sed 's/aarch64/aarch_64/') && \
   curl -LO https://github.com/protocolbuffers/protobuf/releases/download/v3.20.1/protoc-3.20.1-linux-$ARCH.zip && \
   unzip protoc-3.20.1-linux-$ARCH.zip -d $HOME/.local

RUN apt-get update && \
    apt-get install -y protobuf-compiler

COPY ./ /root
COPY test-files /root/share/images
COPY program /root/program
WORKDIR "/root"

CMD sh -c "/opt/cartesi/bin/jsonrpc-remote-cartesi-machine 127.0.0.1:50051 & sleep 2 && cargo run --release"

