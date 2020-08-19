import {
  BuidlerRuntimeEnvironment,
  DeployFunction,
} from "@nomiclabs/buidler/types";

import { ethers } from "@nomiclabs/buidler";

const LoggerJson = require("@cartesi/logger/build/contracts/Logger.json");
const VGInstantiatorJson = require("@cartesi/arbitration/build/contracts/VGInstantiator.json");
const StepJson = require("@cartesi/machine-solidity-step/build/contracts/Step.json");

const func: DeployFunction = async (bre: BuidlerRuntimeEnvironment) => {
  const { deployments, getNamedAccounts } = bre;
  const { deploy } = deployments;
  const { deployer } = await getNamedAccounts();
  const network_id = await ethers.provider.send('net_version', []);

  const LoggerAddress = LoggerJson.networks[network_id].address;
  const VGAddress = VGInstantiatorJson.networks[network_id].address;
  const StepAddress = StepJson.networks[network_id].address;

  await deploy("Descartes", {
    from: deployer,
    log: true,
    args: [LoggerAddress, VGAddress, StepAddress],
  });
};


export default func;
export const tags = ["Descartes"];
