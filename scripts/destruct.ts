/**
 * destruct: deactivates the Cartesi Compute computation for a given index (default index = 0)
 *
 * Basic usage
 * - npx hardhat run --network localhost --no-compile destruct.ts
 *
 * Parametrization (setting env variables)
 * - "index": controls which Cartesi Compute computation to deactivate (default is 0)
 */
import hre from "hardhat";

async function main() {
    const { ethers } = hre;
    const { CartesiCompute } = await hre.deployments.all();

    // retrieves deployed CartesiCompute instance based on its address
    const cartesi_compute = await ethers.getContractAt(
        "CartesiCompute",
        CartesiCompute.address
    );

    let index = "0";
    if (process.env.index) {
        index = process.env.index;
    }
    console.log("");
    console.log("Destructing computation using index '" + index + "'\n");

    const tx = await cartesi_compute.destruct(index);
    console.log(
        `Destruction request successful with index '${index}' (tx: ${tx.hash} ; blocknumber: ${tx.blockNumber})\n`
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
