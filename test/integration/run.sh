#!/bin/bash -x

FULL_PATH=$(dirname $(realpath $0))
DESCARTES_DIR=$(dirname $(dirname $FULL_PATH))

echo $DESCARTES_DIR
echo $FULL_PATH

# ensure flashdrive directories are created by the user and not a Docker's root user
mkdir -p $DESCARTES_DIR/dapp_data_0/flashdrive
mkdir -p $DESCARTES_DIR/dapp_data_1/flashdrive

cd $DESCARTES_DIR;

wait-for-url() {
    echo "wait-for-url $1"
    timeout -s TERM 300 bash -c \
    'while [[ "$(curl -s -o /dev/null -L -w ''%{http_code}'' ${0})" != "200" ]];\
    do echo "Waiting for ${0}" && sleep 2;\
    done' ${1}
    echo "OK!"
}

jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml | docker-compose -f - up --build --no-color &> logs.txt&
wait-for-url http://localhost:8545


# testing HelloWorld
# TODO: removing test for now because of Machine Manager intermittent bug when running many jobs in parallel
# https://github.com/cartesi-corp/machine-manager/issues/46
# ./scripts/helloworld/build-cartesi-machine.sh ./machines
# npx hardhat run --network localhost --no-compile ./scripts/helloworld/instantiate.ts

# testing Calculator
./scripts/calculator/build-cartesi-machine.sh ./machines
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate.ts
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-logger.ts
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-provider.ts

# testing IPFS
./scripts/ipfs/run.sh
./scripts/ipfs/run-large-1M.sh
./scripts/ipfs/run-logger-fallback.sh


npx hardhat run --network localhost --no-compile ./test/integration/wait-results.ts
exitStatus=$?

jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml | docker-compose -f - down -v

exit $exitStatus
