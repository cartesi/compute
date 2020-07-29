const { ethers } = require("@nomiclabs/buidler");
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

const snapshotTaker = (provider) => {
  return async function takeSnapshot() {
    const payload = {
      method: "evm_snapshot",
      params: [],
      jsonrpc: "2.0",
      id: 0,
    };
    const response = await axios.post(provider._buidlerProvider._url, payload);
    const id = response.data.result;
    return async function revert() {
      const payload = {
        method: "evm_revert",
        params: [id],
        jsonrpc: "2.0",
        id: 0,
      };
      const response = await axios.post(
        provider._buidlerProvider._url,
        payload
      );
    };
  };
};

const advanceTime = async (provider, seconds) => {
  const payload = {
    method: "evm_increaseTime",
    params: [seconds],
    jsonrpc: "2.0",
    id: 0,
  };
  await axios.post(provider._buidlerProvider._url, payload);
};

const getBlockTimestampByHash = async (provider, hash) => {
  const payload = {
    method: "eth_getBlockByHash",
    params: [hash, false],
    jsonrpc: "2.0",
    id: 0,
  };
  const response = await axios.post(provider._buidlerProvider._url, payload);
  return ethers.BigNumber.from(response.data.result.timestamp).toNumber()
};
module.exports = {
  driveMatcher,
  snapshotTaker,
  advanceTime,
  getBlockTimestampByHash,
};
