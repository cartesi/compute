#!/bin/bash -x

FULL_PATH=$(dirname $(realpath $0))
CARTESI_COMPUTE_DIR=$(dirname $(dirname $FULL_PATH))

# ensure flashdrive directories are created by the user and not a Docker's root user
mkdir -p $CARTESI_COMPUTE_DIR/dapp_data_0/flashdrive
mkdir -p $CARTESI_COMPUTE_DIR/dapp_data_1/flashdrive

cd $CARTESI_COMPUTE_DIR;

wait-for-url() {
    echo "wait-for-url $1"
    timeout -s TERM 300 bash -c \
    'while [[ "$(curl -s -o /dev/null -L -w ''%{http_code}'' ${0})" != "200" ]];\
    do echo "Waiting for ${0}" && sleep 2;\
    done' ${1}
    echo "OK!"
}

jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml | docker-compose -f - up --build --no-color 2>&1 | tee logs.txt &

wait-for-url http://localhost:8545

docker image ls

# downloading cartesi machine binaries
./scripts/download-images.sh ./images

# testing HelloWorld
echo "Executing helloworld test"
./scripts/helloworld/build-cartesi-machine.sh ./images ./machines
npx hardhat run --network localhost --no-compile ./scripts/helloworld/instantiate.ts

# testing Calculator
echo "Executing calculator test"
./scripts/calculator/build-cartesi-machine.sh ./images ./machines
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate.ts

echo "Executing calculator test with logger"
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-logger.ts

echo "Executing calculator test with logger provider 0"
export PROVIDER=0x0000000000000000000000000000000000000000
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-logger.ts
unset PROVIDER

echo "Executing calculator test with provider"
npx hardhat run --network localhost --no-compile ./scripts/calculator/instantiate-provider.ts

# testing IPFS
echo "Testing IPFS"
./scripts/ipfs/run.sh
echo "Testing IPFS large 1m"
./scripts/ipfs/run-large-1M.sh
echo "Testing IPFS large 8m"
./scripts/ipfs/run-large-8M.sh
echo "Testing IPFS logger fallback"
./scripts/ipfs/run-logger-fallback.sh
echo "Testing direct IPFS node injection"
./scripts/ipfs/run-no-provider.sh

# waiting for resuls
echo "Waiting for results"
npx hardhat run --network localhost --no-compile ./test/integration/wait-results.ts
exitStatus=$?

echo "Done, turning off container"

jinja2 -D num_players=2 -D image=$DOCKERIMAGE docker-compose-template.yml < /dev/null | docker-compose -f - down -v

exit $exitStatus
