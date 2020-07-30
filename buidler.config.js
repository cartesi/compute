const project = process.env.PROJECT_ID;
const mnemonic = process.env.MNEMONIC || "placeholder";

const network = (name, network_id, url=`https://${name}.infura.io/v3/${project}`) => ({
  url,
  accounts: {
    mnemonic,
    count: 100,
  },
  chainId: network_id,
});

usePlugin("@nomiclabs/buidler-waffle");
usePlugin("@nomiclabs/buidler-solpp");
usePlugin('solidity-coverage')

module.exports = {
  defaultNetwork: "development",
  networks: {
    development: {
      url: "http://127.0.0.1:8545"
    },
    buidlerevm: {
      // See its defaults
    },
    coverage: {
      url: "http://127.0.0.1:8555", // <-- If you change this, also set the port option in .solcover.js.
      // chainId: "*", // not set means it's ignored
      gas: 0xfffffffffff, // <-- Use this high gas value
      gasPrice: 0x01      // <-- Use this low gas price
    },
    ropsten: network("ropsten", 3),
    kovan: network("kovan", 42),
    rinkeby: network("rinkeby", 4),
    matic_testnet: network("matic_testnet", 15001, 'https://testnetv3.matic.network'),
  },
  solpp: {
    defs: {
      BUILD_TEST: process.argv.includes('test') || process.argv.includes('coverage'),
    }
  },
  solc: {
    version: "0.5.16",
     // See the solidity docs for advice about optimization and evmVersion
     optimizer: {
      enabled: true,
      runs: 200,
    },
  },
};
