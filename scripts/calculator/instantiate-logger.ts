/**
 * Calculator instantiate
 *
 * Basic usage
 * - npx hardhat run --network localhost --no-compile calculator/instantiate.ts
 *
 * Parametrization (setting env variables)
 * - "data": defines mathematical expression to evaluate (default is "2^71 + 36^12")
 */
import hre from "hardhat";

const config = {
    finalTime: JSON.parse(process.env.FINAL_TIME || "1e13"),
    roundDuration: Number.parseInt(process.env.ROUND_DURATION || "51"),
    provider: process.env.PROVIDER,
};

async function main() {
    console.log("Loaded Configs: ", JSON.stringify(config, null, 2));

    const { ethers, getNamedAccounts } = hre;
    const { alice, bob } = await getNamedAccounts();

    // retrieves Cartesi Compute and Logger deployed contracts
    const cartesi_compute = await ethers.getContract("CartesiCompute");
    const logger = await ethers.getContract("Logger");

    let data = "2^71 + 36^12";
    if (process.env.data) {
        data = process.env.data;
    }
    console.log("");
    console.log(
        `Instantiating "Calculator" for data "${data}" using the Logger...\n`
    );

    // submits data to the logger
    const dataUint8Array = ethers.utils.toUtf8Bytes(data);
    // TEMP: always using "2^71 + 36^12" in hex form for testing
    const txLogger = await logger.calculateMerkleRootFromData(5, [
        "0x325E3731202B2033",
        "0x365E313200000000",
    ]);

    // retrieves root hash for submitted logger data
    const logRoot = await new Promise((resolve) => {
        logger.on("MerkleRootCalculatedFromData", (_1, _2, root, _4) =>
            resolve(root)
        );
    });
    console.log(
        `Submitted data to logger with root hash '${logRoot}' (tx: ${txLogger.hash} ; blocknumber: ${txLogger.blockNumber})\n`
    );

    // defines input drive
    let provider = config.provider;
    if (provider === undefined) {
        provider = alice;
    }
    const input = {
        position: "0x9000000000000000",
        driveLog2Size: 5,
        directValue: ethers.utils.formatBytes32String(""),
        loggerIpfsPath: ethers.utils.formatBytes32String(""),
        loggerRootHash: logRoot,
        waitsProvider: false,
        needsLogger: true,
        provider: provider,
    };

    // instantiates cartesi_compute computation
    const tx = await cartesi_compute.instantiate(
        // final time
        config.finalTime,
        // template hash
        "0x838e3ee2307ceda86e8c9275bcb57378de61d785e5fc6e377dacf00f389c3adb",
        // output position
        "0xa000000000000000",
        // output log2 size
        10,
        // round duration
        config.roundDuration,
        [alice, bob],
        [input]
    );

    // retrieves created computation's index
    const index = await new Promise((resolve) => {
        cartesi_compute.on("CartesiComputeCreated", (index) => resolve(index));
    });

    console.log(
        `Instantiation successful with index '${index}' (tx: ${tx.hash} ; blocknumber: ${tx.blockNumber})\n`
    );
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
