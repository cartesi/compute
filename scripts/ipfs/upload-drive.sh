#!/bin/bash

# general definitions
CARTESI_IPFS_DOCKER=cartesi/ipfs-server:0.2.0
IPFS_COMPOSE_NETWORK=descartes_ipfs
IPFS_SERVICE_ADDRESS="ipfs_0:50051"  # 0 is alice
INPUT_DRIVE_FILENAME=$loggerRootHash

output=$(docker run \
  --network=$IPFS_COMPOSE_NETWORK \
  --entrypoint "/opt/cartesi/bin/test_client" \
  -v `pwd`:/opt/cartesi/srv/descartes \
  --rm  $CARTESI_IPFS_DOCKER \
  -address $IPFS_SERVICE_ADDRESS -mode add -argument /opt/cartesi/srv/descartes/flashdrive/$INPUT_DRIVE_FILENAME 2>&1)

IPFS_PATH=${output:96:52}
echo "New IPFS Path: $IPFS_PATH"
IPFS_PATH_DATA=$(echo -n $IPFS_PATH | xxd -p)
echo "IPFS path bytes 0x$IPFS_PATH_DATA"

export IPFS_PATH
export IPFS_PATH_DATA