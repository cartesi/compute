#!/bin/bash

# general definitions
MACHINES_DIR=.
MACHINE_TEMP_DIR=__temp_cartesi_machine
MACHINE_MANAGER_IMAGE=cartesi/machine-manager:master
MACHINE_IMAGES_DIR=./images
CONTAINER_NAME=cartesi-machine-builder

# check cartesi machine binaries dir was provided
if [ -z "$1" ] || [ ! -d "$1" ] ; then
  echo "Cartesi machine images path not defined or does not exist"
  exit 1
else
  MACHINE_IMAGES_DIR=$( cd -- "$1" &> /dev/null && pwd )
fi

# set machines directory to specified path if provided
if [ ! -z "$2" ]; then
  MACHINES_DIR=$2
fi

# removes machine temp store directory if it exists
if [ -d "$MACHINE_TEMP_DIR" ]; then
  rm -r $MACHINE_TEMP_DIR
fi

# remove builder container if it exists
if docker inspect $CONTAINER_NAME > /dev/null 2>&1; then
  docker rm -f $CONTAINER_NAME > /dev/null;
fi

# builds machine (running with 0 cycles)
# - initial (template) hash is printed on screen
# - machine is stored in temporary directory
docker run -v $MACHINE_IMAGES_DIR:/opt/cartesi/share/images \
  --name $CONTAINER_NAME $MACHINE_MANAGER_IMAGE \
  cartesi-machine \
    --max-mcycle=0 \
    --append-rom-bootargs="single=yes" \
    --initial-hash \
    --store="/tmp/$MACHINE_TEMP_DIR" \
    --flash-drive="label:input,length:1<<12" \
    --flash-drive="label:output,length:1<<12" \
    -- $'dd status=none if=$(flashdrive input) | lua -e \'print((string.unpack("z",  io.read("a"))))\' | bc | dd status=none of=$(flashdrive output)'

# copy stored machine directory from the container to host machine
docker cp $CONTAINER_NAME:/tmp/$MACHINE_TEMP_DIR $MACHINE_TEMP_DIR
# remove builder container
docker rm -f $CONTAINER_NAME > /dev/null;

# calculate the stored machine hash
MACHINE_TEMPLATE_HASH=$(docker run --rm \
  -v `pwd`/$MACHINE_TEMP_DIR:/tmp/$MACHINE_TEMP_DIR $MACHINE_MANAGER_IMAGE \
  /opt/cartesi/bin/cartesi-machine-stored-hash /tmp/$MACHINE_TEMP_DIR)

# moves stored machine to a folder within $MACHINES_DIR named after the machine's hash
mv $MACHINE_TEMP_DIR $MACHINES_DIR/$MACHINE_TEMPLATE_HASH
ls -alR $MACHINES_DIR
