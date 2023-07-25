#!/bin/bash
cartesi-machine \
  --max-mcycle=0 \
  --store="simple-linux-program" \
  -- echo "Hello, world!"
