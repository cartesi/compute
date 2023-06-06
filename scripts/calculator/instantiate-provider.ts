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
};

async function main() {
    console.log("Loaded Configs: ", JSON.stringify(config, null, 2));

    const { ethers, getNamedAccounts } = hre;
    const { alice, bob, charlie, dave } = await getNamedAccounts();

    let num_peers = 2;
    if (process.env.num_peers) {
        num_peers = Number.parseInt(process.env.num_peers);
    }
    const peers = [alice, bob, charlie, dave].slice(0, num_peers);

    // retrieves Cartesi Compute deployed contract
    const cartesi_compute = await ethers.getContract("CartesiCompute");

    let data = "2^71 + 36^12";
    if (process.env.data) {
        data = process.env.data;
    }
    console.log("");
    console.log(
        `Instantiating "Calculator" for data "${data}" with ${peers.length} peers...\n`
    );

    // defines input drive
    const input = {
        position: "0x90000000000000",
        driveLog2Size: 12,
        directValue: ethers.utils.formatBytes32String(""),
        loggerIpfsPath: ethers.utils.formatBytes32String(""),
        loggerRootHash: ethers.utils.formatBytes32String(""),
        waitsProvider: true,
        needsLogger: false,
        provider: alice,
    };

    // instantiates cartesi_compute computation
    const tx = await cartesi_compute.instantiate(
        // final time
        config.finalTime,
        // template hash
        "0xa3b304cd520dc7ffb3ae9f7f46200b1a2c474235c12209c05753a7e5170c3449",
        // output position
        "0xa0000000000000",
        // output log2 size
        10,
        // round duration
        config.roundDuration,
        peers,
        [input],
        false
    );

    // retrieves created computation's index
    const index = await new Promise((resolve) => {
        cartesi_compute.on("CartesiComputeCreated", (index) => resolve(index));
    });

    console.log(
        `Instantiation successful with index '${index}' (tx: ${tx.hash} ; blocknumber: ${tx.blockNumber})\n`
    );

    // sends provider drive's data
    const drivePromise = new Promise((resolve) => {
        cartesi_compute.on("DriveInserted", (index, drive) => resolve(drive));
    });
    const txDrive = await cartesi_compute.provideDirectDrive(
        index,
        ethers.utils.toUtf8Bytes(data)
    );
    console.log("Inserted drive");
    const drive = await drivePromise;
    console.log(
        `Inserted provider drive '${JSON.stringify(drive)}' (tx: ${
            txDrive.hash
        } ; blocknumber: ${txDrive.blockNumber})\n`
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
