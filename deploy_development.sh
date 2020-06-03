#!/bin/sh

# this script will compile and migrate not only this project contracts, but also all dependent contracts, in the correct order

# remove build directory to do a clean build
rm ./build/ -rf
cd node_modules/@cartesi/util && truffle migrate --network development && cd ../../../
cd node_modules/@cartesi/arbitration && truffle migrate --network development && cd ../../../
cd node_modules/@cartesi/machine-solidity-step && truffle migrate --network development && cd ../../../
cd node_modules/@cartesi/logger && truffle migrate --network development && cd ../../../
truffle migrate --network development
