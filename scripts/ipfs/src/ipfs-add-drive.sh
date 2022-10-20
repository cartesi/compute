#!/bin/bash

# general definitions
CARTESI_IPFS_DOCKER=cartesi/ipfs-server:0.2.0
IPFS_COMPOSE_NETWORK=ipfs
IPFS_SERVICE_ADDRESS="ipfs_0:50051"  # 0 is alice
INPUT_DRIVE_FILENAME=$LOGGER_ROOT_HASH

# set ipfs service to specified address if provided
if [ $1 ]; then
  IPFS_SERVICE_ADDRESS=$1
fi

output=$(docker run \
  --network=$IPFS_COMPOSE_NETWORK \
  --entrypoint "/opt/cartesi/bin/test_client" \
  -v `pwd`:/opt/cartesi/srv/cartesi_compute \
  --rm  $CARTESI_IPFS_DOCKER \
  -address $IPFS_SERVICE_ADDRESS -mode add -argument /opt/cartesi/srv/cartesi_compute/flashdrive/$INPUT_DRIVE_FILENAME 2>&1)

# searches for string 'ipfs_path:"', after which comes the desired value
output=${output#*\ipfs_path:\"}

# IPFS path is retrieved by selecting the next 52 characters: "/ipfs/<46-char-hash>"
IPFS_PATH=${output:0:52}
echo "New IPFS Path: $IPFS_PATH"

export IPFS_PATH
