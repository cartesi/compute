/**
 * HelloWorld instantiate
 *
 * Basic usage
 * - npx hardhat run --network localhost --no-compile helloworld/instantiate.ts
 */
import hre from "hardhat";

async function sleep(seconds: number): Promise<any> {
    return new Promise((resolve) => {
        setTimeout(resolve, seconds * 1000);
    });
}

async function main() {
    const failedTests: string[] = [];
    const { ethers, getNamedAccounts } = hre;
    const { CartesiCompute } = await hre.deployments.all();

    const { alice, bob, charlie, dave } = await getNamedAccounts();

    let num_peers = 2;
    if (process.env.num_peers) {
        num_peers = Number.parseInt(process.env.num_peers);
    }
    const peers = [alice, bob, charlie, dave].slice(0, num_peers);

    // retrieves deployed Cartesi Compute instance based on its address
    const cartesi_compute = await ethers.getContractAt(
        "CartesiCompute",
        CartesiCompute.address
    );

    console.log("");
    console.log(`Grabbing information on deployed Cartesi Compute\n`);

    const filter = cartesi_compute.filters.CartesiComputeCreated();

    const events = await cartesi_compute.queryFilter(filter, 0, "latest");

    console.log(`Found ${events.length} Cartesi Compute instances running`);
    const activeInstances: number[] = [];
    events.forEach((e) => {
        const arg = e.args || { _index: 0 };
        activeInstances.push(arg._index.toNumber());
    });

    while (activeInstances.length > 0) {
        let i = 0;
        while (i < activeInstances.length) {
            const instance = activeInstances[i];
            const state = await cartesi_compute.getCurrentState(instance);
            const parsedState = ethers.utils.parseBytes32String(state);
            console.log(`Instance ${instance} is at state ${parsedState}`);

            switch (parsedState) {
                case "ConsensusResult":
                    console.log(`====================== execution result ====================`);
                    const result = await cartesi_compute.getResult(instance)
                    console.log(ethers.utils.toUtf8String(result[3]))
                    console.log(`============================================================`);
                    activeInstances.splice(i, 1);
                    break;
                case "ChallengerWon":
                    activeInstances.splice(i, 1);
                    failedTests.push(
                        `Cartesi Compute test at index ${instance} has failed. Finished at ${parsedState} state.`
                    );
                    break;
                case "ClaimerWon":
                    activeInstances.splice(i, 1);
                    failedTests.push(
                        `Cartesi Compute test at index ${instance} has failed. Finished at ${parsedState} state.`
                    );
                    break;
                case "ProviderMissedDeadline":
                    activeInstances.splice(i, 1);
                    failedTests.push(
                        `Cartesi Compute test at index ${instance} has failed. Finished at ${parsedState} state.`
                    );
                    break;
                case "ClaimerMissedDeadline":
                    activeInstances.splice(i, 1);
                    failedTests.push(
                        `Cartesi Compute test at index ${instance} has failed. Finished at ${parsedState} state.`
                    );
                    break;
                default:
                    i++;
            }
        }
        console.log("------------- sleeping 20s");
        await sleep(20);
    }
    if (failedTests.length > 0) {
        throw `Some tests failed. Here is the compiled messages: ${JSON.stringify(
            failedTests,
            null,
            2
        )}`;
    }
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.

const timeout = new Promise((_, rejects) => {
    let time = 30 * 1000 * 60; // 30min
    if (process.env.timeout) {
        time = Number.parseInt(process.env.timeout, 10) * 1000 * 60;
    }
    setTimeout(() => {
        rejects(
            `The test has timedout after waiting for ${
                time / (1000 * 60)
            } min. Check the logs.`
        );
    }, time);
});

Promise.race([main(), timeout])
    .then(() => process.exit(0))
    .catch((error) => {
        console.error(error);
        process.exit(1);
    });
