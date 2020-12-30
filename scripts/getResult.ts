/**
 * getResult: returns the Descartes result for a given index (default index = 0)
 * 
 * Basic usage
 * - npx hardhat run --network localhost --no-compile getResult.ts
 * 
 * Parametrization (setting env variables)
 * - "index": controls which Descartes computation to query (default is 0)
 */
import hre from "hardhat";

async function main() {
  const { ethers } = hre;
  const { Descartes } = await hre.deployments.all();

  // retrieves deployed Descartes instance based on its address
  const descartes = await ethers.getContractAt("Descartes", Descartes.address);

  let index = "0";
  if (process.env.index) {
    index = process.env.index;
  }
  console.log("");
  console.log("Getting result using index '" + index + "'\n");

  const ret = await descartes.getResult(index);
  console.log("Full result: " + JSON.stringify(ret));
  if (ret["3"]) {
      console.log(`Result value as string: ${ethers.utils.toUtf8String(ret["3"])}`);
  }
  console.log("");
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
