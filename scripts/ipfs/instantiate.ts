import hre from "hardhat";

const config = {
  ipfsPath: process.env.IPFS_PATH || "",
  loggerRootHash: process.env.LOGGER_ROOT_HASH || "",
  machineTemplateHash: process.env.MACHINE_TEMPLATE_HASH || "",
}

Object.entries(config).forEach(([key, value]) => {
  if(value.length === 0) {
    console.error(`${key} could not be found in environment vars`, config);
    process.exit(-1);
  } 
});

async function main() {

  console.log('Loaded Configs: ', JSON.stringify(config, null, 2));

  const { ethers, getNamedAccounts } = hre;
  const { Descartes } = await hre.deployments.all();
  const { alice, bob } = await getNamedAccounts();

  const descartes = await ethers.getContractAt("Descartes", Descartes.address); // as unknown as Descartes;
  const aDrive = {
    position: "0x9000000000000000",
    driveLog2Size: 12,
    // bytes of print(math.sin(1))
    directValue: ethers.utils.formatBytes32String(""),
    //  bytes of "ipfs_path:"/ipfs/QmVX3WoKxjy96wjCJXtkdgvpirT86MsncX6J9UQBc4XXSJ" (content: "print(math.sin(1))")
    loggerIpfsPath: ethers.utils.hexlify(ethers.utils.toUtf8Bytes(config.ipfsPath.replace(/\s+/g, ''))),
    //`0x${config.loggerIpfsPath.replace(/\s+/g, '')}`,
    // hash of print(math.sin(1))
    loggerRootHash: `0x${config.loggerRootHash}`,
    waitsProvider: false,
    needsLogger: true,
    provider: alice,
  };

  const tx = await descartes.instantiate(
    // final time: 1e11 gives us ~50 seconds for completing the computation itself
    1e11,
    // template hash
    `0x${config.machineTemplateHash}`,
    // output position
    "0xa000000000000000",
    // output log2 size
    5,
    // round duration
    51,
    [alice, bob],
    [aDrive]
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
