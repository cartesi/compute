#!/bin/bash
cartesi-machine \
  --max-mcycle=0 \
  --store="programs/simple-linux-program" \
  -- echo "Hello, world!"
