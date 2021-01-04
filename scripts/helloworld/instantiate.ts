/**
 * HelloWorld instantiate
 * 
 * Basic usage
 * - npx hardhat run --network localhost --no-compile helloworld/instantiate.ts
 */
import hre from "hardhat";

async function main() {
  const { ethers, getNamedAccounts } = hre;
  const { Descartes } = await hre.deployments.all();
  
  const {alice, bob} = await getNamedAccounts();

  // retrieves deployed Descartes instance based on its address
  const descartes = await ethers.getContractAt("Descartes", Descartes.address);

  console.log("");
  console.log(`Instantiating "HelloWorld"...\n`);

  // instantiates descartes computation
  const tx = await descartes.instantiate(
    // final time: 1e11 gives us ~50 seconds for completing the computation itself
    1e11,
    // template hash
    "0x65e171ad372e1ec29d1d02d0446e666ac06b54f908103c28009bec9b0e566344",
    // output position
    "0x9000000000000000",
    // output log2 size
    5,
    // round duration
    51,
    [alice, bob],
    []
  );

  // retrieves created computation's index
  const index = await new Promise(resolve => {
    descartes.on("DescartesCreated", index => resolve(index))
  });

  console.log(`Instantiation successful with index '${index}' (tx: ${tx.hash} ; blocknumber: ${tx.blockNumber})\n`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
