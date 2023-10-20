#!/bin/bash
cartesi-machine \
  --no-root-flash-drive \
  --rom-image="./bins/rom-v0.16.0.bin" \
  --ram-image="./bins/linux-5.15.63-ctsi-2.bin" \
  --max-mcycle=0 \
  --store="programs/simple-linux-program" \
  -- echo "Hello, world!"
