#!/bin/bash

# general definitions
CARTESI_COMPUTE_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )"/.. &> /dev/null && pwd )
SCRIPT_DIR=$CARTESI_COMPUTE_DIR/scripts
MACHINE_IMAGES_DIR=$CARTESI_COMPUTE_DIR/images

# set machine images directory to specified path if provided
if [ ! -z "$1" ]; then
  MACHINE_IMAGES_DIR=$1
fi

# create machine images directory if it does not exist
if [ ! -d "$MACHINE_IMAGES_DIR" ]; then
    mkdir -p $MACHINE_IMAGES_DIR
fi

echo "Downloading cartesi-machine rom, kernel and rootfs..."
wget -q -nc -i $SCRIPT_DIR/dependencies -P $MACHINE_IMAGES_DIR
pushd $MACHINE_IMAGES_DIR &> /dev/null && shasum -c $SCRIPT_DIR/shasumfile && popd &> /dev/null
ln -s linux-5.15.63-ctsi-2.bin $MACHINE_IMAGES_DIR/linux.bin
ln -s rom-v0.16.0.bin $MACHINE_IMAGES_DIR/rom.bin
ln -s rootfs-v0.17.0.ext2 $MACHINE_IMAGES_DIR/rootfs.ext2
