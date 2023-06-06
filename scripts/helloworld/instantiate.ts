/**
 * HelloWorld instantiate
 *
 * Basic usage
 * - npx hardhat run --network localhost --no-compile helloworld/instantiate.ts
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

    // retrieves deployed Cartesi Compute instance based on its address
    const cartesi_compute = await ethers.getContract("CartesiCompute");

    console.log("");
    console.log(`Instantiating "HelloWorld" with ${peers.length} peers...\n`);

    // instantiates cartesi_compute computation
    const tx = await cartesi_compute.instantiate(
        // final time
        config.finalTime,
        // template hash
        "0xde7d607a6489e4ab55df472e3774e0d5e0543122d3f157c67b1933ca3e98cf21",
        // output position
        "0x90000000000000",
        // output log2 size
        5,
        // round duration
        config.roundDuration,
        peers,
        [],
        false
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
