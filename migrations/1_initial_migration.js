var Migrations = artifacts.require("Migrations");

module.exports = function(deployer, network, accounts) {
  if (network == "geth") {
    web3.eth.personal.unlockAccount(accounts[0], "private_network", 15000);
  }
  deployer.deploy(Migrations);
};
