#!/bin/bash

# general definitions
FULL_PATH=$(dirname $(realpath $0))
CARTESI_COMPUTE_DIR=$(dirname $(dirname $FULL_PATH))

if [ -z "$DRIVE_LOG2_SIZE" ]; then
  DRIVE_LOG2_SIZE=12
fi

# set base cartesi compute directory to specified path if provided
if [ $1 ]; then
  CARTESI_COMPUTE_DIR=$1
fi

# Build the cartesi machine
. $FULL_PATH/src/build-cartesi-machine.sh $CARTESI_COMPUTE_DIR/images $CARTESI_COMPUTE_DIR/machines

# Prepare the drive with the calculation script and
. $FULL_PATH/src/build-flash-drive.sh $CARTESI_COMPUTE_DIR

# Do not add the drive to IPFS and just set an invalid path
export IPFS_PATH=/ipfs-invalid-path
echo "Using invalid path '$IPFS_PATH' to force Logger fallback"

# Instantiate cartesi compute and start the process
npx hardhat run $FULL_PATH/instantiate.ts --no-compile --network localhost
