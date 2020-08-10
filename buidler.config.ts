import { usePlugin } from "@nomiclabs/buidler/config";

const project = process.env.PROJECT_ID;
const mnemonic = process.env.MNEMONIC || "placeholder";

const network = (name:string, network_id:number, url=`https://${name}.infura.io/v3/${project}`) => ({
  url,
  accounts: {
    mnemonic,
    count: 100,
  },
  chainId: network_id,
});

usePlugin("@nomiclabs/buidler-solpp");
usePlugin("@nomiclabs/buidler-waffle");
usePlugin("@nodefactory/buidler-typechain");
usePlugin('solidity-coverage')
usePlugin("buidler-deploy");

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
  namedAccounts: {
    deployer: {
        default: 0
    },
  },
  solpp: {
    defs: {
      BUILD_TEST: process.argv.includes('test') || process.argv.includes('coverage'),
    }
  },
  typechain: {
    outDir: "src/types",
    target: "ethers-v5",
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
