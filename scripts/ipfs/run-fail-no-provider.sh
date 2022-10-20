#!/bin/bash

# general definitions
FULL_PATH=$(dirname $(realpath $0))
CARTESI_COMPUTE_DIR=$(dirname $(dirname $FULL_PATH))

if [ -z "$DRIVE_LOG2_SIZE" ]; then
  DRIVE_LOG2_SIZE=12
fi

# set base Cartesi Compute directory to specified path if provided
if [ $1 ]; then
  CARTESI_COMPUTE_DIR=$1
fi

# Build the cartesi machine
. $FULL_PATH/src/build-cartesi-machine.sh $CARTESI_COMPUTE_DIR/images $CARTESI_COMPUTE_DIR/machines

# Prepare the drive with the calculation script and
. $FULL_PATH/src/build-flash-drive.sh $CARTESI_COMPUTE_DIR

# Do not add the drive to IPFS and just set an invalid path, while setting provider to address 0
export IPFS_PATH=/ipfs-invalid-path
export PROVIDER=0x0000000000000000000000000000000000000000
echo "Using invalid path '$IPFS_PATH' and provider 'address(0)' to force failure"

# Instantiate Cartesi Compute and start the process
npx hardhat run $FULL_PATH/instantiate.ts --no-compile --network localhost
