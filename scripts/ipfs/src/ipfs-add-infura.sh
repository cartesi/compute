#!/bin/bash

# submits file to infura
filename=$FLASHDRIVE_BASE_DIR/$FLASHDRIVE_PATH/$LOGGER_ROOT_HASH
output=$(curl -X POST -s -F file=@$filename "https://ipfs.infura.io:5001/api/v0/add?pin=true")

# searches for string 'Hash":"', after which comes the desired value
output=${output#*Hash\":\"}

# IPFS path is retrieved by '/ipfs/' followed by the 46-character hash
IPFS_PATH="/ipfs/${output:0:46}"
echo "New IPFS Path (uploaded to Infura): $IPFS_PATH"

export IPFS_PATH
