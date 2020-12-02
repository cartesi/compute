const { ethers } = require("hardhat");
const { expect, use } = require("chai");
const { solidity } = require("ethereum-waffle");
const axios = require("axios");

use(solidity);

const driveMatcher = (expected, toEqual) => {
  expect(expected.position).to.equal(toEqual.position, "drive.position");
  expect(expected.driveLog2Size).to.equal(
    toEqual.driveLog2Size,
    "drive.driveLog2Size"
  );
  expect(expected.directValue).to.equal(
    toEqual.directValue,
    "drive.directValue"
  );
  expect(expected.loggerRootHash).to.equal(
    toEqual.loggerRootHash,
    "drive.loggerRootHash"
  );
  expect(expected.provider).to.equal(toEqual.provider, "drive.provider");
  expect(expected.waitsProvider).to.equal(
    toEqual.waitsProvider,
    "drive.waitsProvider"
  );
  expect(expected.needsLogger).to.equal(
    toEqual.needsLogger,
    "drive.needsLogger"
  );
};

const snapshotTaker = provider => {
  return async function takeSnapshot() {
    const id = await provider.send("evm_snapshot", []);
    return async function revert() {
      const response = await provider.send("evm_revert", [id]);
    };
  };
};

const advanceTime = async (provider, seconds) => {
  await provider.send("evm_increaseTime", [seconds]);
};

const getBlockTimestampByHash = async (provider, hash) => {
  const response = await provider.send("eth_getBlockByHash", [hash, false]);
  return ethers.BigNumber.from(response.timestamp).toNumber();
};
module.exports = {
  driveMatcher,
  snapshotTaker,
  advanceTime,
  getBlockTimestampByHash
};
