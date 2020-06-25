#!/bin/sh

# exit when any command fails
set -e

if [ -z "${MNEMONIC}" ]; then
    echo "No MNEMONIC, waiting for key file at /opt/cartesi/etc/keys/"
    dockerize -wait file:///opt/cartesi/etc/keys/keys_done -timeout ${ETHEREUM_TIMEOUT}

    export CARTESI_CONCERN_KEY=$(cat /opt/cartesi/etc/keys/private_key)
    export ACCOUNT_ADDRESS=$(cat /opt/cartesi/etc/keys/account)
else
    echo "Initializing key and account from MNEMONIC"
    export CARTESI_CONCERN_KEY=$(wagyu ethereum import-hd --mnemonic "${MNEMONIC}" --derivation "m/44'/60'/0'/0/${ACCOUNT_INDEX}" --json | jq -r '.[0].private_key')
    export ACCOUNT_ADDRESS=$(wagyu ethereum import-hd --mnemonic "${MNEMONIC}" --derivation "m/44'/60'/0'/0/${ACCOUNT_INDEX}" --json | jq -r '.[0].address')
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
    -wait tcp://${ETHEREUM_HOST}:${ETHEREUM_PORT} \
    -timeout ${ETHEREUM_TIMEOUT}

echo "Creating configuration file at /opt/cartesi/etc/descartes/config.yaml with account ${ACCOUNT_ADDRESS}"
envsubst < /opt/cartesi/etc/descartes/config-template.yaml > /opt/cartesi/etc/descartes/config.yaml
cat /opt/cartesi/etc/descartes/config.yaml

echo "Starting dispatcher"
/opt/cartesi/bin/descartes --config_path /opt/cartesi/etc/descartes/config.yaml --working_path /opt/cartesi/srv/descartes
