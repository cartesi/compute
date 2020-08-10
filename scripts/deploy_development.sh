#!/bin/sh

# this script will compile and migrate not only this project contracts, but also all dependent contracts, in the correct order

cd ..
# remove build directory to do a clean build
rm ./build/ -rf
root=$PWD

cd ./node_modules/@cartesi/util && npx truffle migrate --network development && cd $root
cd ./node_modules/@cartesi/arbitration && npx truffle migrate --network development && cd $root
cd ./node_modules/@cartesi/machine-solidity-step && npx truffle migrate --network development && cd $root
cd ./node_modules/@cartesi/logger && npx truffle migrate --network development && cd $root
npx buidler deploy