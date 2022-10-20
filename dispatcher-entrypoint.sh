#!/bin/sh

# exit when any command fails
set -e

if [ -z "${KEY_SERVER_HOST}" ]; then
    if [ -n "${CONCERN_SEMAPHORE}" ]; then
        # wait for key file and read from them
        echo "Waiting for key signal at ${CONCERN_SEMAPHORE}"
        dockerize -wait ${CONCERN_SEMAPHORE} -timeout ${ETHEREUM_TIMEOUT}

        if [ -f "/opt/cartesi/etc/keys/private_key" ]; then
            export CARTESI_CONCERN_KEY=$(cat /opt/cartesi/etc/keys/private_key)
        fi

        if [ -f "/opt/cartesi/etc/keys/account" ]; then
            export ACCOUNT_ADDRESS=$(cat /opt/cartesi/etc/keys/account)
        fi

    elif [ -n "${MNEMONIC}" ]; then
        echo "Initializing key and account from MNEMONIC"
        export CARTESI_CONCERN_KEY=$(wagyu ethereum import-hd --mnemonic "${MNEMONIC}" --derivation "m/44'/60'/0'/0/${ACCOUNT_INDEX}" --json | jq -r '.[0].private_key')
        export ACCOUNT_ADDRESS=$(wagyu ethereum import-hd --mnemonic "${MNEMONIC}" --derivation "m/44'/60'/0'/0/${ACCOUNT_INDEX}" --json | jq -r '.[0].address')
    fi
else
    export CARTESI_CONCERN_KEY=$(curl -s "http://${KEY_SERVER_HOST}:4000/get-key-info?id=cartesi_compute&type=secp256k1" | jq -r .info.privateKey)
    export ACCOUNT_ADDRESS=$(curl -s "http://${KEY_SERVER_HOST}:4000/get-key-info?id=cartesi_compute&type=secp256k1" | jq -r .info.address)
fi

# wait for deployment if env is set
if [ -n "${DEPLOYMENT_SEMAPHORE}" ]; then
    echo "Waiting for blockchain deployment..."
    dockerize -wait ${DEPLOYMENT_SEMAPHORE} -timeout ${ETHEREUM_TIMEOUT}
fi

echo "Waiting for services..."
dockerize \
    -wait tcp://${MACHINE_MANAGER_HOST}:${MACHINE_MANAGER_PORT} \
    -wait tcp://${LOGGER_HOST}:${LOGGER_PORT} \
    -wait tcp://${IPFS_HOST}:${IPFS_PORT} \
    -wait tcp://${ETHEREUM_HOST}:${ETHEREUM_PORT} \
    -timeout ${ETHEREUM_TIMEOUT}


if [ -z "${CONCERN_SEMAPHORE}" ] && [ -z "${MNEMONIC}" && [ -z "${ACCOUNT_ADDRESS}"] ]; then
    echo "No mnemonic or file set, using external signer"
    export ACCOUNT_ADDRESS=$(curl -X POST --data '{"jsonrpc":"2.0","method":"eth_accounts","params":[],"id":1}' http://${ETHEREUM_HOST}:${ETHEREUM_PORT} | jq -r '.result[0]')
fi

echo "Creating configuration file at /opt/cartesi/etc/cartesi_compute/config.yaml with account ${ACCOUNT_ADDRESS}"
envsubst < /opt/cartesi/etc/cartesi_compute/config-template.yaml > /opt/cartesi/etc/cartesi_compute/config.yaml
cat /opt/cartesi/etc/cartesi_compute/config.yaml

echo "Starting dispatcher"
/opt/cartesi/bin/cartesi_compute --config_path /opt/cartesi/etc/cartesi_compute/config.yaml --working_path /opt/cartesi/srv/cartesi_compute
