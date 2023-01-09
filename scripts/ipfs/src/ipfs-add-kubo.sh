#!/bin/bash

# general definitions
CARTESI_IPFS_DOCKER=curlimages/curl:7.87.0
IPFS_COMPOSE_NETWORK=ipfs
IPFS_SERVICE_ADDRESS="kubo_0:5001"  # 0 is alice
INPUT_DRIVE_FILENAME=$LOGGER_ROOT_HASH

# set ipfs service to specified address if provided
if [ $1 ]; then
  IPFS_SERVICE_ADDRESS=$1
fi

output=$(docker run \
  --network=$IPFS_COMPOSE_NETWORK \
  -v `pwd`:/data \
  --rm $CARTESI_IPFS_DOCKER \
  -F file=@/data/dapp_data_0/flashdrive/$INPUT_DRIVE_FILENAME http://$IPFS_SERVICE_ADDRESS/api/v0/add 2>&1)

# searches for string 'Hash":"', after which comes the desired value
output=${output#*Hash\":\"}

# IPFS path is retrieved by '/ipfs/' followed by the 46-character hash
IPFS_PATH="/ipfs/${output:0:46}"

echo "New IPFS Path (uploaded to Kubo instance): $IPFS_PATH"

export IPFS_PATH
