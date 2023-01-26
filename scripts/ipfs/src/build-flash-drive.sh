#!/bin/bash

# general definitions
FLASHDRIVE_BASE_DIR=.
FLASHDRIVE_PATH=dapp_data_0/flashdrive
CARTESI_IPFS_DOCKER=cartesi/ipfs-server:0.3.0


# set flashdrive base directory to specified path if provided
if [ $1 ]; then
  FLASHDRIVE_BASE_DIR=$1
fi

# set flashdrive path to specified path if provided
if [ $2 ]; then
  FLASHDRIVE_PATH=$2
fi

RANDOM=$(date +%s%N)
echo -e -n "#!/usr/bin/lua\nprint(math.sin($RANDOM))" > input_drive

size=$(echo "2^$DRIVE_LOG2_SIZE" | bc)
truncate -s $size input_drive

LOGGER_ROOT_HASH="$(docker run \
  --entrypoint "/opt/cartesi/bin/merkle-tree-hash" \
  -v `pwd`:/mount \
  --rm  $CARTESI_IPFS_DOCKER \
  --page-log2-size=$DRIVE_LOG2_SIZE --tree-log2-size=$DRIVE_LOG2_SIZE  --input=/mount/input_drive)"

mkdir -p $FLASHDRIVE_BASE_DIR/$FLASHDRIVE_PATH
cp input_drive $FLASHDRIVE_BASE_DIR/$FLASHDRIVE_PATH/$LOGGER_ROOT_HASH


echo "New loggerRootHash: 0x$LOGGER_ROOT_HASH"
rm input_drive

export LOGGER_ROOT_HASH
