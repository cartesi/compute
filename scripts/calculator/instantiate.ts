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
    const cartesi_compute = await ethers.getContract("CartesisCompute");

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
        position: "0x9000000000000000",
        driveLog2Size: 5,
        directValue: ethers.utils.toUtf8Bytes(data),
        loggerIpfsPath: ethers.utils.formatBytes32String(""),
        loggerRootHash: ethers.utils.formatBytes32String(""),
        waitsProvider: false,
        needsLogger: false,
        provider: alice,
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
        peers,
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
