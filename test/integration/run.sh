#!/bin/bash -x

FULL_PATH=$(dirname $(realpath $0))
DESCARTES_DIR=$(dirname $(dirname $FULL_PATH))

echo $DESCARTES_DIR
echo $FULL_PATH

cd $DESCARTES_DIR;

wait-for-url() {
    echo "wait-for-url $1"
    timeout -s TERM 90 bash -c \
    'while [[ "$(curl -s -o /dev/null -L -w ''%{http_code}'' ${0})" != "200" ]];\
    do echo "Waiting for ${0}" && sleep 2;\
    done' ${1}
    echo "OK!"
}

jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml | docker-compose -f - up --build --no-color &> logs.txt&
wait-for-url http://localhost:8545



./scripts/helloworld/build-cartesi-machine.sh ./machines
./scripts/calculator/build-cartesi-machine.sh ./machines
./scripts/ipfs/run.sh


npx hardhat run --network localhost --no-compile ./scripts/helloworld/instantiate.ts
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate.ts
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-logger.ts

npx hardhat run --network localhost --no-compile ./test/integration/wait-results.ts


jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml | docker-compose -f - down -v
