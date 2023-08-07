#!/bin/bash
cartesi-machine \
  --no-root-flash-drive \
  --rom-image="program/bins/bootstrap.bin" \
  --ram-image="program/bins/rv64ui-p-addi.bin" \
  --uarch-ram-image="/opt/cartesi/share/images/uarch-ram.bin" \
  --uarch-ram-length=0x1000000 \
  --max-mcycle=0 \
  --store="simple-program"