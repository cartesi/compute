import hre from "hardhat";

const config = {
    ipfsPath: process.env.IPFS_PATH || "",
    loggerRootHash: process.env.LOGGER_ROOT_HASH || "",
    machineTemplateHash: process.env.MACHINE_TEMPLATE_HASH || "",
    driveLog2Size: process.env.DRIVE_LOG2_SIZE || "12",
    finalTime: JSON.parse(process.env.FINAL_TIME || "1e13"),
    roundDuration: Number.parseInt(process.env.ROUND_DURATION || "51"),
    provider: process.env.PROVIDER,
};

Object.entries(config).forEach(([key, value]) => {
    if (value === "") {
        console.error(`${key} could not be found in environment vars`, config);
        process.exit(-1);
    }
});

async function main() {
    console.log("Loaded Configs: ", JSON.stringify(config, null, 2));

    const { ethers, getNamedAccounts } = hre;
    const { alice, bob, charlie, dave } = await getNamedAccounts();

    let num_peers = 2;
    if (process.env.num_peers) {
        num_peers = Number.parseInt(process.env.num_peers);
    }
    const peers = [alice, bob, charlie, dave].slice(0, num_peers);
    console.log(
        `Configured peers ${JSON.stringify(peers)} from named accounts.`
    );

    let provider = config.provider;
    if (provider === undefined) {
        provider = alice;
    }

    // retrieves CartesiCompute deployed contract
    const cartesi_compute = await ethers.getContract("CartesiCompute");

    // creates drive
    const aDrive = {
        position: "0x9000000000000000",
        driveLog2Size: config.driveLog2Size,
        // bytes of print(math.sin(1))
        directValue: ethers.utils.formatBytes32String(""),
        //  bytes of "/ipfs/QmVX3WoKxjy96wjCJXtkdgvpirT86MsncX6J9UQBc4XXSJ" (content: "print(math.sin(1))")
        loggerIpfsPath: ethers.utils.hexlify(
            ethers.utils.toUtf8Bytes(config.ipfsPath)
        ),
        // hash of 'print(math.sin(1))' padded to drive's size
        loggerRootHash: `0x${config.loggerRootHash}`,
        waitsProvider: false,
        needsLogger: true,
        provider: provider,
    };

    console.log("");
    console.log(`Instantiating "IPFS" with ${peers.length} peers...\n`);

    const tx = await cartesi_compute.instantiate(
        // final time
        config.finalTime,
        // template hash
        `0x${config.machineTemplateHash}`,
        // output position
        "0xa000000000000000",
        // output log2 size
        5,
        // round duration
        config.roundDuration,
        peers,
        [aDrive]
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
