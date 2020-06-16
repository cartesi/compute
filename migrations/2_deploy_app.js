const contract = require("@truffle/contract");

const Logger = contract(require("@cartesi/logger/build/contracts/Logger.json"));
const VGInstantiator = contract(require("@cartesi/arbitration/build/contracts/VGInstantiator.json"));
const Step = contract(require("@cartesi/machine-solidity-step/build/contracts/Step.json"));

const Descartes = artifacts.require("Descartes");

module.exports = function(deployer) {
    Logger.setNetwork(deployer.network_id);
    VGInstantiator.setNetwork(deployer.network_id);
    Step.setNetwork(deployer.network_id);

    deployer.deploy(Descartes, Logger.address, VGInstantiator.address, Step.address);
};
