const Descartes = artifacts.require("Descartes");

module.exports = function(deployer) {
    deployer.deploy(Descartes);
};
