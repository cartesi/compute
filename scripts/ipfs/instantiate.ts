import hre from "hardhat";

async function main() {
  const { ethers, getNamedAccounts } = hre;
  const { Descartes } = await hre.deployments.all();
  const { alice, bob } = await getNamedAccounts();

  const descartes = await ethers.getContractAt("Descartes", Descartes.address); // as unknown as Descartes;
  const aDrive = {
    position: "0x9000000000000000",
    driveLog2Size: 12,
    // bytes of print(math.sin(1))
    directValue: "0x7072696e74286d6174682e73696e28312929",
    //  bytes of "ipfs_path:"/ipfs/QmUHFdQV9vpJkHacarem5Pf71NdsW9dgEee1fN3ndYayxx" (content: "print(math.sin(1))")
    loggerIpfsPath:
      "0x2f697066732f516d565833576f4b786a793936776a434a58746b6467767069725438364d736e6358364a3955514263345858534a",
    // hash of print(math.sin(1))
    loggerRootHash:"0x7b9d938fb0c8fb24ece1a9eb89e1a2ab180ce562561d4c589c642c6a9cf9e1ee",
    waitsProvider: false,
    needsLogger: true,
    provider: alice,
  };

  const tx = await descartes.instantiate(
    // final time
    1e13,
    // template hash
    "0x254b2f36dd335ee10a32cb60ded8e063ba62d009fdb01ce84861bd5b52593320",
    // output position
    "0x9000000000000000",
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
