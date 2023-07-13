import { HardhatUserConfig, task } from "hardhat/config";
import { HttpNetworkUserConfig } from "hardhat/types";

import "@nomiclabs/hardhat-waffle";
import "hardhat-typechain";
import "hardhat-deploy";
import "hardhat-deploy-ethers";

import "@nomiclabs/hardhat-solpp";
// import "solidity-coverage"; @dev WIP this plugin is not updated to hardhat yet

// This is a sample hardhat task. To learn how to create your own go to
// https://hardhat.dev/guides/create-task.html
task("accounts", "Prints the list of accounts", async (taskArgs, bre) => {
    const accounts = await bre.ethers.getSigners();
    for (const account of accounts) {
        console.log(await account.getAddress());
    }
});

// read MNEMONIC from file or from env variable
let mnemonic = process.env.MNEMONIC;

const infuraNetwork = (
    network: string,
    chainId?: number,
    gas?: number
): HttpNetworkUserConfig => {
    return {
        url: `https://${network}.infura.io/v3/${process.env.PROJECT_ID}`,
        chainId,
        gas,
        accounts: mnemonic ? { mnemonic } : undefined,
    };
};

const config: HardhatUserConfig = {
    networks: {
        hardhat: mnemonic ? { accounts: { mnemonic } } : {},
        localhost: {
            url: "http://localhost:8545",
            accounts: mnemonic ? { mnemonic } : undefined,
        },
        coverage: {
            url: "http://localhost:8555", // <-- If you change this, also set the port option in .solcover.js.
            // chainId: "*", // not set means it's ignored
            gas: 0xfffffffffff, // <-- Use this high gas value
            gasPrice: 0x01, // <-- Use this low gas price
        },
        mainnet: infuraNetwork("mainnet", 1, 6283185),
        arbitrum_one: {
            url: process.env.RPC_URL || "https://arb1.arbitrum.io/rpc",
            accounts: mnemonic ? { mnemonic } : undefined,
        },
        goerli: infuraNetwork("goerli", 5, 6283185),
        matic_testnet: infuraNetwork("polygon-mumbai", 80001),
        bsc_testnet: {
            url: "https://data-seed-prebsc-2-s2.binance.org:8545/",
            chainId: 97,
            accounts: mnemonic ? { mnemonic } : undefined,
        },
        avax_testnet: {
            url: "https://api.avax-test.network/ext/bc/C/rpc",
            chainId: 0xa869,
            accounts: mnemonic ? { mnemonic } : undefined,
        },
    },
    solidity: {
        compilers: [
            {
                version: "0.7.4",
                settings: {
                    optimizer: {
                        enabled: true,
                    },
                },
            },
            {
                version: "0.8.15",
                settings: {
                    optimizer: {
                        enabled: true,
                    },
                },
            },
        ],
    },
    paths: {
        artifacts: "artifacts",
        deploy: "deploy",
        deployments: "deployments",
    },
    external: {
        contracts: [
            {
                artifacts: "node_modules/@cartesi/util/export/artifacts",
                deploy: "node_modules/@cartesi/util/dist/deploy",
            },
            {
                artifacts: "node_modules/@cartesi/arbitration/export/artifacts",
                deploy: "node_modules/@cartesi/arbitration/dist/deploy",
            },
            {
                artifacts: "node_modules/@cartesi/logger/export/artifacts",
                deploy: "node_modules/@cartesi/logger/dist/deploy",
            },
            {
                deploy:
                    "node_modules/@cartesi/machine-solidity-step/dist/deploy",
                artifacts:
                    "node_modules/@cartesi/machine-solidity-step/export/artifacts",
            },
        ],
        deployments: {
            goerli: [
                "node_modules/@cartesi/util/deployments/goerli",
                "node_modules/@cartesi/arbitration/deployments/goerli",
                "node_modules/@cartesi/logger/deployments/goerli",
                "node_modules/@cartesi/machine-solidity-step/deployments/goerli",
            ],
            matic_testnet: [
                "node_modules/@cartesi/util/deployments/matic_testnet",
                "node_modules/@cartesi/arbitration/deployments/matic_testnet",
                "node_modules/@cartesi/logger/deployments/matic_testnet",
                "node_modules/@cartesi/machine-solidity-step/deployments/matic_testnet",
            ],
            bsc_testnet: [
                "node_modules/@cartesi/util/deployments/bsc_testnet",
                "node_modules/@cartesi/arbitration/deployments/bsc_testnet",
                "node_modules/@cartesi/logger/deployments/bsc_testnet",
                "node_modules/@cartesi/machine-solidity-step/deployments/bsc_testnet",
            ],
            avax_testnet: [
                "node_modules/@cartesi/util/deployments/avax_testnet",
                "node_modules/@cartesi/arbitration/deployments/avax_testnet",
                "node_modules/@cartesi/logger/deployments/avax_testnet",
                "node_modules/@cartesi/machine-solidity-step/deployments/avax_testnet",
            ],
        },
    },
    solpp: {
        defs: {
            BUILD_TEST:
                process.argv.includes("test") ||
                process.argv.includes("coverage"),
        },
    },
    typechain: {
        outDir: "src/types",
        target: "ethers-v5",
    },
    namedAccounts: {
        deployer: {
            default: 0,
        },
        user: {
            default: 0,
        },
        worker: {
            default: 1,
        },
        alice: {
            default: 0,
        },
        bob: {
            default: 1,
        },
        charlie: {
            default: 2,
        },
        dave: {
            default: 3,
        },
        proxy: {
            default: 1,
        },
    },
};

export default config;
