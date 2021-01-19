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
import LoggerJson from "@cartesi/logger/export/artifacts/Logger.json"

async function main() {
  const { ethers, getNamedAccounts } = hre;
  const { Descartes, Logger } = await hre.deployments.all();
  
  const {alice, bob} = await getNamedAccounts();


  // retrieves deployed Descartes instance based on its address
  const descartes = await ethers.getContractAt("Descartes", Descartes.address);
  // retrieves deployed Logger instance based on its address
  let [signer] = await ethers.getSigners();
  const logger = new ethers.Contract(Logger.address, LoggerJson.abi, signer);

  let data = "2^71 + 36^12";
  if (process.env.data) {
    data = process.env.data;
  }
  console.log("");
  console.log(`Instantiating "Calculator" for data "${data}" using the Logger...\n`);

  // submits data to the logger
  const dataUint8Array = ethers.utils.toUtf8Bytes(data);
  // TEMP: always using "2^71 + 36^12" in hex form for testing
  const txLogger = await logger.calculateMerkleRootFromData(5, ['0x325E3731202B2033','0x365E313200000000']);

  // retrieves root hash for submitted logger data 
  const logRoot = await new Promise(resolve => {
    logger.on("MerkleRootCalculatedFromData", (_1, _2, root, _4) => resolve(root))
  });
  console.log(`Submitted data to logger with root hash '${logRoot}' (tx: ${txLogger.hash} ; blocknumber: ${txLogger.blockNumber})\n`);

  // defines input drive
  const input = {
    position: "0x9000000000000000",
    driveLog2Size: 5,
    directValue: ethers.utils.formatBytes32String(""),
    loggerIpfsPath: ethers.utils.formatBytes32String(""),
    loggerRootHash: logRoot,
    waitsProvider: false,
    needsLogger: true,
    provider: alice
  };

  // instantiates descartes computation
  const tx = await descartes.instantiate(
    // final time: 1e11 gives us ~50 seconds for completing the computation itself
    1e11,
    // template hash
    "0xa278371ed8d52efa6aba9f825ba8130d2604b363b3ceb51c1bd3a210f400fd8a",
    // output position
    "0xa000000000000000",
    // output log2 size
    10,
    // round duration
    51,
    [alice, bob],
    [input]
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
