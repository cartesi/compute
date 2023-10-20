#!/bin/bash
cartesi-machine \
  --no-root-flash-drive \
  --rom-image="./bins/bootstrap.bin" \
  --ram-image="./bins/rv64ui-p-addi.bin" \
  --max-mcycle=0 \
  --store="programs/simple-program"
