/**
 * getCurrentState: returns the Descartes current state for a given index (default index = 0)
 *
 * Basic usage
 * - npx hardhat run --network localhost --no-compile getCurrentState.ts
 *
 * Parametrization (setting env variables)
 * - "index": controls which Descartes computation to query (default is 0)
 */
import hre from "hardhat";

async function main() {
    const { ethers } = hre;
    const { Descartes } = await hre.deployments.all();

    // retrieves deployed Descartes instance based on its address
    const descartes = await ethers.getContractAt(
        "Descartes",
        Descartes.address
    );

    let index = "0";
    if (process.env.index) {
        index = process.env.index;
    }
    console.log("");
    console.log("Getting current state using index '" + index + "'\n");

    const ret = await descartes.getCurrentState(index);
    console.log(`Current state: ${ethers.utils.toUtf8String(ret)}`);
    console.log("");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
