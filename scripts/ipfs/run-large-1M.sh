#!/bin/bash

echo "Using large file: 1MB (DRIVE_LOG2_SIZE = 20)"
export DRIVE_LOG2_SIZE=20

FULL_PATH=$(dirname $(realpath $0))
. $FULL_PATH/run.sh
