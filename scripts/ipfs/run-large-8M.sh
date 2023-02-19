#!/bin/bash

echo "Using large file: 8MB (DRIVE_LOG2_SIZE = 23)"
export DRIVE_LOG2_SIZE=23

FULL_PATH=$(dirname $(realpath $0))
. $FULL_PATH/run.sh
