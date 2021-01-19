#!/bin/bash

# general definitions
FULL_PATH=$(dirname $(realpath $0))
DESCARTES_DIR=$(dirname $(dirname $FULL_PATH))

# set base descartes directory to specified path if provided
if [ $1 ]; then
  DESCARTES_DIR=$1
fi

# Build the cartesi machine 
. $FULL_PATH/src/build-cartesi-machine.sh $DESCARTES_DIR/machines

# Prepare the drive with the calculation script and
. $FULL_PATH/src/build-flash-drive.sh $DESCARTES_DIR

# Add the drive to IPFS
. $FULL_PATH/src/ipfs-add-drive.sh

# Instantiate descartes and start the process
npx hardhat run $FULL_PATH/instantiate.ts --no-compile --network localhost