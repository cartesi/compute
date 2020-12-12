// We require the Hardhat Runtime Environment explicitly here. This is optional 
// but useful for running the script in a standalone fashion through `node <script>`.
// When running the script with `hardhat run <script>` you'll find the Hardhat
// Runtime Environment's members available in the global scope.
import hre from "hardhat";
import { Descartes } from "../src/types/Descartes";

async function main() {
  const { ethers, getNamedAccounts, artifacts } = hre;
  const { Descartes } = await hre.deployments.all();
  // Hardhat always runs the compile task when running scripts through it. 
  // If this runs in a standalone fashion you may want to call compile manually 
  // to make sure everything is compiled
  // await hre.run('compile');
  
  const {alice, bob} = await getNamedAccounts();
  
  // We get the contract to deployed
  // const [signer] =  await ethers.getSigners();
  // console.log(signer);
  // const descartesArt = await artifacts.readArtifact("Descartes");
  // const descartes = new ethers.Contract(Descartes.address, descartesArt.abi, signer);
  const descartes = await ethers.getContractAt(
    "Descartes",
    Descartes.address,
    );// as unknown as Descartes;
    // console.log(descartes);
  // const aDrive = {
  //   position: "0x9000000000000000",
  //   driveLog2Size: 5,
  //   // bytes of print(math.sin(1))
  //   directValue: "0x7072696e74286d6174682e73696e28312929",
  //   // bytes of "/ipfs/QmeCXnPMGMymihD1QvvKQdwhp7wRp3iXzbz5jQ5XyPCN1w" (content: "print(math.sin(1))")
  //   loggerIpfsPath: "0x2f697066732f516d6543586e504d474d796d696844315176764b5164776870377752703369587a627a356a5135587950434e3177",
  //   // hash of print(math.sin(1))
  //   loggerRootHash: "0xa87f79b5149218496af4d722798f46cdb1a15b12928ad05618892e5b3f999062",
  //   waitsProvider: false,
  //   needsLogger: true,
  //   provider: claimer,
  // };
//
  const tx = await descartes.instantiate(
    // final time
    1e13,
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
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error);
    process.exit(1);
  });
