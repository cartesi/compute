import {
  HardhatRuntimeEnvironment,
  DeployFunction,
} from "hardhat/types";

import { ethers } from "hardhat";

// @dev WIP we need to stabilize dependencies first
// const LoggerJson = require("@cartesi/logger/build/contracts/Logger.json");
// const VGInstantiatorJson = require("@cartesi/arbitration/build/contracts/VGInstantiator.json");
// const StepJson = require("@cartesi/machine-solidity-step/build/contracts/Step.json");

const func: DeployFunction = async (bre: HardhatRuntimeEnvironment) => {
  const { deployments, getNamedAccounts } = bre;
  const { deploy, get } = deployments;
  const { deployer } = await getNamedAccounts();
  const network_id = await ethers.provider.send('net_version', []);

  const LoggerAddress = "0xD89C67422a23EC3BB3a59A4d37E46C833155e41c"; //LoggerJson.networks[network_id].address;
  const VGAddress = "0xD89C67422a23EC3BB3a59A4d37E46C833155e41c"; //VGInstantiatorJson.networks[network_id].address;
  const StepAddress = "0xD89C67422a23EC3BB3a59A4d37E46C833155e41c"; //StepJson.networks[network_id].address;

  const Merkle = await get('Merkle');
  await deploy("Descartes", {
    from: deployer,
    log: true,
    libraries: {
      Merkle: Merkle.address,
    },
    args: [LoggerAddress, VGAddress, StepAddress],
  });
};


export default func;
export const tags = ["Descartes"];
