#!/bin/bash

# general definitions
FLASHDRIVE_BASE_DIR=.
CARTESI_IPFS_DOCKER=cartesi/ipfs-server:0.2.0


# set machines directory to specified path if provided
if [ $1 ]; then
  FLASHDRIVE_BASE_DIR=$1
fi


echo -e -n "#!/usr/bin/lua\nprint(math.sin(1))" > input_drive

directValue="0x$(xxd -p input_drive)"
echo -e "New directValue: $directValue"

truncate -s %4096 input_drive



LOGGER_ROOT_HASH="$(docker run \
  --entrypoint "/opt/cartesi/bin/merkle-tree-hash" \
  -v `pwd`:/mount \
  --rm  $CARTESI_IPFS_DOCKER \
  --page-log2-size=3 --tree-log2-size=12  --input=/mount/input_drive)"

mkdir -p $FLASHDRIVE_BASE_DIR/dapp_data_0/flashdrive
cp input_drive $FLASHDRIVE_BASE_DIR/dapp_data_0/flashdrive/$LOGGER_ROOT_HASH


echo "New loggerRootHash: 0x$LOGGER_ROOT_HASH"
rm input_drive

export LOGGER_ROOT_HASH