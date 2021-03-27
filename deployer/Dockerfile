FROM node:14-alpine

# install dockerize, as we need to wait on ganache to be responding
RUN apk add --no-cache \
    build-base \
    git \
    openssl \
    python \
#     python-dev \
    py-pip

# ENV DOCKERIZE_VERSION v0.6.1
# RUN wget https://github.com/jwilder/dockerize/releases/download/$DOCKERIZE_VERSION/dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
#     && tar -C /usr/local/bin -xzvf dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz \
#     && rm dockerize-alpine-linux-amd64-$DOCKERIZE_VERSION.tar.gz

ENV BASE /opt/cartesi

WORKDIR $BASE/deployer
COPY yarn.lock .
COPY package.json .
COPY hardhat.config.ts .
COPY tsconfig.json .
COPY contracts .
COPY deploy .

RUN yarn 

ENV ETHEREUM_HOST "hardhatnet"
ENV ETHEREUM_PORT "8545"
ENV ETHEREUM_TIMEOUT "10s"
ENV ETHEREUM_NETWORK "localhost"
ENV DOCKER true

EXPOSE 8545  

WORKDIR $BASE/share/blockchain

CMD ["npx", "hardhat",  "node" ]
