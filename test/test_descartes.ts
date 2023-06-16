import * as e from "ethers";
import { ethers, waffle, deployments } from "hardhat";
import { expect } from "chai";
import { MockContract } from "@ethereum-waffle/mock-contract";
import { CartesiCompute } from "../src/types/CartesiCompute";

const { deployMockContract } = waffle;

const {
  driveMatcher,
  snapshotTaker,
  advanceTime,
  getBlockTimestampByHash,
} = require("./utils");

const deployCartesiCompute = async ({
  logger,
  vg,
  step,
}: {
  logger?: string;
  vg?: string;
  step?: string;
} = {}): Promise<CartesiCompute> => {
  const CartesiComputeFactory = await ethers.getContractFactory("CartesiCompute");
  const cartesi_compute = await CartesiComputeFactory.deploy(logger, vg, step);
  await cartesi_compute.deployed();
  return cartesi_compute as unknown as CartesiCompute;
};

describe("Cartesi Compute tests", () => {
  let mainSigner: e.Signer;
  let mainSignerAddress: string;
  let claimer: e.Signer;
  let claimerAddress: string;
  let challenger: e.Signer;
  let challengerAddress: string;
  let accounts: e.Signer[];
  let finalTime = 300;
  let templateHash = ethers.constants.HashZero;
  let outputPosition = 0;
  let roundDuration = 50;
  let outputLog2Size = 3;
  let output = "0x" + "00".repeat(8);
  let aDrive = {
    position: 0,
    driveLog2Size: 3,
    directValue: "0x" + "00".repeat(8),
    driveRootHash: ethers.constants.HashZero,
    directDrive: true,
  };
  let cartesi_compute: CartesiCompute;
  let takeSnapshot: Function;
  let mockVG: MockContract;
  let mockLogger: MockContract;
  let instantiateTimestamp: number;

  before(async () => {
    const { Step, VGInstantiator, Logger } = await deployments.all();
    accounts = await ethers.getSigners();
    [mainSigner, claimer, challenger] = accounts;
    takeSnapshot = snapshotTaker(mainSigner.provider);
    mainSignerAddress = await mainSigner.getAddress();
    claimerAddress = await claimer.getAddress();
    challengerAddress = await challenger.getAddress();
    mockVG = await deployMockContract(mainSigner, VGInstantiator.abi);
    cartesi_compute = await deployCartesiCompute({
      vg: mockVG.address,
      step: Step.address,
    });
  });

  describe("Cartesi Compute Straight Pass", () => {
    it("Should instantiate correctly", async () => {
      /* Instantiate and provides all the necessary information to end this
      // transaction in "WaitingClaim"
      */
      const tx = cartesi_compute.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        outputLog2Size,
        roundDuration,
        [claimerAddress, challengerAddress],
        [aDrive],
      );
      await expect(tx).to.emit(cartesi_compute, "CartesiComputeCreated").withArgs(0);
      // save 'now' used in other pieces of the contract
      const timestamp = await getBlockTimestampByHash(
        mainSigner.provider,
        (await tx).blockHash
      );
      instantiateTimestamp = timestamp;

      const tx2 = await cartesi_compute.getState(0, mainSignerAddress);
      expect(tx2[0][0]).to.equal(finalTime);
      expect(tx2[0][1]).to.equal(timestamp + 40 + roundDuration); // lastMoveTime  = now + timeToStartMachine(40)
      expect(tx2[0][2]).to.equal(outputPosition);
      expect(tx2).to.include.deep.members([
        [
          ethers.constants.AddressZero, // @TODO order inconsistency
          claimerAddress,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.utils.formatBytes32String("WaitingClaim"), // currentState
        ],
        "0x",
      ]);

      expect(tx2[4]).to.have.length(1);
      driveMatcher(tx2[4][0], aDrive);
    });

    it("Should respond isConcerned correctly", async () => {
      let res = await cartesi_compute.isConcerned(0, claimerAddress);
      expect(res).to.equal(true);

      res = await cartesi_compute.isConcerned(0, challengerAddress);
      expect(res).to.equal(true);

      res = await cartesi_compute.isConcerned(0, mainSignerAddress);
      expect(res).to.equal(false);

      await expect(cartesi_compute.isConcerned(1, claimerAddress)).to.be.revertedWith(
        "Index not instantiated"
      );
    });

    it("Should succeed to abortByDeadline -ClaimerMissedDeadline-", async () => {
      let revertSnapshot = await takeSnapshot();
      await advanceTime(mainSigner.provider, finalTime);
      await cartesi_compute.abortByDeadline(0);
      const tx = await cartesi_compute.getCurrentState(0);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("ClaimerMissedDeadline")
      );
      await revertSnapshot();
    });

    it("Should transition on submit claim", async () => {
      let tx = cartesi_compute.submitClaim(
        0,
        ethers.constants.HashZero,
        [[ethers.constants.HashZero]],
        ethers.constants.HashZero,
        [ethers.constants.HashZero]
      );
      await expect(tx).to.be.revertedWith("Sender must be a claimer");

      tx = cartesi_compute
        .connect(claimer)
        .submitClaim(
          0,
          ethers.constants.HashZero,
          [],
          ethers.constants.HashZero,
          [ethers.constants.HashZero]
        );
      await expect(tx).to.be.revertedWith(
        "Claimed drive number should match claimed siblings number"
      );

      tx = cartesi_compute
        .connect(claimer)
        .submitClaim(
          0,
          ethers.constants.HashZero,
          [[ethers.constants.HashZero]],
          ethers.constants.HashZero,
          [ethers.constants.HashZero]
        );
      await expect(tx).to.be.revertedWith(
        "Output length doesn't match output log2 size"
      );

      tx = cartesi_compute.connect(claimer).submitClaim(
        0,
        "0xa00d9e556b6a50ea387769f51017057482fae0e7ed2e117a2056d4b3e6031430", // a wrong claimed final hash
        [[ethers.constants.HashZero]],
        output,
        [ethers.constants.HashZero]
      );
      await expect(tx).to.be.revertedWith(
        "Output not in final hash"
      );

      tx = cartesi_compute
        .connect(claimer)
        .submitClaim(
          0,
          ethers.constants.HashZero,
          [[ethers.constants.HashZero]],
          output,
          [ethers.constants.HashZero]
        );

      await expect(tx)
        .to.emit(cartesi_compute, "ClaimSubmitted")
        .withArgs(0, ethers.constants.HashZero);

      const tx2 = await cartesi_compute.getState(0, mainSignerAddress);
      expect(tx2[0][2]).to.equal(outputPosition);
      expect(tx2).to.include.deep.members([
        [ethers.constants.AddressZero, claimerAddress],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.utils.formatBytes32String("WaitingConfirmationDeadline"), // currentState
        ],
        output,
      ]);
    });

    it("Should transition to ConsensusResult after confirm", async () => {
      const revertSnapshot = await takeSnapshot();

      let tx = cartesi_compute
        .connect(challenger)
        .confirm(0);

      await expect(tx)
        .to.emit(cartesi_compute, "Confirmed")
        .withArgs(0, challengerAddress)
        .to.emit(cartesi_compute, "CartesiComputeFinished")
        .withArgs(0, ethers.utils.formatBytes32String("ConsensusResult"));

      const tx2 = await cartesi_compute.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("ConsensusResult")
      );

      await revertSnapshot();
    });

    it("Should abortByDeadline correctly", async () => {
      let tx = cartesi_compute.abortByDeadline(0);
      await expect(tx).to.be.revertedWith(
        ""
      );

      const revertSnapshot = await takeSnapshot();
      await advanceTime(mainSigner.provider, finalTime);

      tx = cartesi_compute.abortByDeadline(0);
      await expect(tx).not.to.be.reverted;

      const tx2 = await cartesi_compute.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("ConsensusResult")
      );

      const tx3 = await cartesi_compute.getResult(0);
      expect(tx3).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = Object.values(tx3);
      expect(resultReady).to.be.true;
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal(output);

      await revertSnapshot();
    });

    it("Should get empty getSubInstances", async () => {
      let tx = await cartesi_compute.getSubInstances(0, mainSignerAddress);
      expect(tx).to.have.length(2);
      expect(tx._addresses).to.be.empty;
      expect(tx._indices).to.be.empty;
    });

    it("Should challenge", async () => {
      let tx = cartesi_compute.challenge(0);
      await expect(tx).to.be.revertedWith("Sender must be a party");

      await mockVG.mock.instantiate.returns(123);
      tx = cartesi_compute.connect(challenger).challenge(0);
      await expect(tx).to.emit(cartesi_compute, "ChallengeStarted").withArgs(0);

      const lastMoveTS = await getBlockTimestampByHash(
        mainSigner.provider,
        (await tx).blockHash
      );

      const tx2 = await cartesi_compute.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("WaitingChallengeResult")
      );

      const getMaxInstanceDuration = 222;
      await mockVG.mock.getMaxInstanceDuration.returns(getMaxInstanceDuration);
      const tx3 = await cartesi_compute.getState(0, claimerAddress);
      expect(tx3).to.have.length(7);
      expect(tx3[0]).to.have.length(4);
      expect(tx3[0][1]).to.be.equal(lastMoveTS + getMaxInstanceDuration + roundDuration);
      expect(tx3[5]).to.have.deep.property("isParty", true);
      expect(tx3[5]).to.have.deep.property("hasVoted", true);
      expect(tx3[5]).to.have.deep.property("hasCheated", false);

      const tx4 = await cartesi_compute.getResult(0);
      expect(tx4).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = Object.values(tx4);
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.true;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal("0x");
    });

    it("Should get vg at getSubInstances", async () => {
      let tx = await cartesi_compute.getSubInstances(0, mainSignerAddress);
      expect(tx).to.have.length(2);
      expect(tx._addresses).to.be.deep.equal([mockVG.address]);
      expect(tx._indices).to.have.length(1);
      expect(tx._indices[0]).to.be.equal(123);
    });

    it("Should winByVG and do both transitions", async () => {
      // ---- Challenger Wins
      let revertSnapshot = await takeSnapshot();
      // not necessary, but future proof in case the check order in the code changes
      await mockVG.mock.stateIsFinishedChallengerWon.returns(true);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(false);

      const tx = await cartesi_compute.winByVG(0);

      await expect(tx)
        .to.emit(cartesi_compute, "CartesiComputeFinished")
        .withArgs(0, ethers.utils.formatBytes32String("ChallengerWon"));

      const tx2 = await cartesi_compute.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("ChallengerWon")
      );
      const tx3 = await cartesi_compute.getResult(0);
      expect(tx3).to.have.length(4);
      let [resultReady, sdkRunning, blameUser, result] = Object.values(tx3);
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(claimerAddress);
      expect(result).to.be.equal("0x");
      await revertSnapshot();

      // ---- Claimer Wins
      revertSnapshot = await takeSnapshot();
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(true);

      const winByVGTx = await cartesi_compute.winByVG(0);

      await expect(winByVGTx)
        .to.emit(cartesi_compute, "CartesiComputeFinished")
        .withArgs(0, ethers.utils.formatBytes32String("ClaimerWon"));

      const tx4 = await cartesi_compute.getCurrentState(0);
      expect(tx4).to.be.equal(ethers.utils.formatBytes32String("ClaimerWon"));
      const tx5 = await cartesi_compute.getResult(0);
      expect(tx5).to.have.length(4);
      [resultReady, sdkRunning, blameUser, result] = Object.values(tx5);
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(challengerAddress);
      expect(result).to.be.equal("0x");

      const lastMoveTS = await getBlockTimestampByHash(
        mainSigner.provider,
        winByVGTx.blockHash
      );

      const tx6 = await cartesi_compute.getState(0, mainSignerAddress);
      expect(tx6).to.have.length(7);
      expect(tx6[0][1]).to.equal(lastMoveTS + 0);

      await revertSnapshot();

      // ---- VG is not finished
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(false);
      await expect(cartesi_compute.winByVG(0)).to.be.revertedWith(
        "VG state not final"
      );
    });
  });
});
