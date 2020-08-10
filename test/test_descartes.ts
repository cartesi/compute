import { ethers } from "@nomiclabs/buidler";
import { expect, use } from "chai";
import { solidity } from "ethereum-waffle";
import { deployMockContract, MockContract } from "@ethereum-waffle/mock-contract";
import * as e from "ethers";
import { Descartes } from "../src/types/Descartes";
const {
  driveMatcher,
  snapshotTaker,
  advanceTime,
  getBlockTimestampByHash,
} = require("./utils");
use(solidity);

const LoggerJson = require("@cartesi/logger/build/contracts/Logger.json");
const VGInstantiatorJson = require("@cartesi/arbitration/build/contracts/VGInstantiator.json");
const StepJson = require("@cartesi/machine-solidity-step/build/contracts/Step.json");

const network_id = 31337; // buidler node chain_id

const deployDescartes = async ({ logger, vg, step }:{ logger?: string, vg?:string, step?:string}={}): Promise<Descartes> => {
  const LoggerAddress = logger || LoggerJson.networks[network_id].address;
  const VGAddress = vg || VGInstantiatorJson.networks[network_id].address;
  const StepAddress = step || StepJson.networks[network_id].address;
  const DescartesFactory = await ethers.getContractFactory("Descartes");
  const descartes = await DescartesFactory.deploy(
    LoggerAddress,
    VGAddress,
    StepAddress
  );
  await descartes.deployed();
  return descartes as Descartes;
};

describe("Descartes tests", () => {
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
  let roundDuration = 0;
  let aDrive = {
    position: 0,
    driveLog2Size: 3,
    directValue: "0x" + "00".repeat(8),
    loggerRootHash: ethers.constants.HashZero,
    waitsProvider: false,
    needsLogger: false,
    provider: "",
  };
  let descartes: Descartes;
  let takeSnapshot: Function;
  let mockVG: MockContract;
  let mockLogger: MockContract;
  let instantiateTimestamp: number;

  before(async () => {
    accounts = await ethers.getSigners();
    [mainSigner, claimer, challenger] = accounts;
    aDrive.provider = await claimer.getAddress();
    takeSnapshot = snapshotTaker(mainSigner.provider);
    mainSignerAddress = await mainSigner.getAddress();
    claimerAddress = await claimer.getAddress();
    challengerAddress = await challenger.getAddress();
    mockVG = await deployMockContract(mainSigner, VGInstantiatorJson.abi);
    mockLogger = await deployMockContract(mainSigner, LoggerJson.abi);
    descartes = await deployDescartes({
      vg: mockVG.address,
      logger: mockLogger.address,
    });
  });

  describe("Descartes Straight Pass", () => {
    it("Should instantiate correctly", async () => {
      /* Instantiate and provides all the necessary information to end this
      // transaction in "WaitingClaim"
      */
      
      const tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimerAddress,
        challengerAddress,
        [aDrive]
      );
      await expect(tx).to.emit(descartes, "DescartesCreated").withArgs(0);
      // save 'now' used in other pieces of the contract
      const timestamp = await getBlockTimestampByHash(
        mainSigner.provider,
        (await tx).blockHash
      );
      instantiateTimestamp = timestamp;

      const tx2 = await descartes.getState(0, mainSignerAddress);
      expect(tx2[0][0]).to.equal(finalTime);
      expect(tx2[0][1]).to.equal(timestamp + 40); // lastMoveTime  = now + timeToStartMachine(40)
      expect(tx2[0][2]).to.equal(outputPosition);
      expect(tx2).to.include.deep.members([
        [
          challengerAddress, // @TODO order inconsistency
          claimerAddress,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.constants.HashZero, // claimedOutput
          ethers.utils.formatBytes32String("WaitingClaim"), // currentState
        ],
      ]);
      expect(tx2[3]).to.have.length(1);
      driveMatcher(tx2[3][0], aDrive);
    });

    it("Should respond isConcerned correctly", async () => {
      let res = await descartes.isConcerned(0, claimerAddress);
      expect(res).to.equal(true);

      res = await descartes.isConcerned(0, challengerAddress);
      expect(res).to.equal(true);

      res = await descartes.isConcerned(0, mainSignerAddress);
      expect(res).to.equal(false);

      await expect(descartes.isConcerned(1, claimerAddress)).to.be.revertedWith("Index not instantiated");
    });

    it("Should succeed to abortByDeadline -ClaimerMissedDeadline-", async () => {
      let revertSnapshot = await takeSnapshot();
      await advanceTime(mainSigner.provider, finalTime);
      await descartes.abortByDeadline(0);
      const tx = await descartes.getCurrentState(0);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("ClaimerMissedDeadline")
      );
      await revertSnapshot();
    });

    it("Should transition on submit claim", async () => {
      let tx = descartes.submitClaim(
        0,
        ethers.constants.HashZero,
        [[ethers.constants.HashZero]],
        ethers.constants.HashZero,
        [ethers.constants.HashZero]
      );
      await expect(tx).to.be.revertedWith("Cannot be called by user");

      tx = descartes
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

      tx = descartes.connect(claimer).submitClaim(
        0,
        "0xa00d9e556b6a50ea387769f51017057482fae0e7ed2e117a2056d4b3e6031430", // a wrong claimed final hash
        [[ethers.constants.HashZero]],
        ethers.constants.HashZero,
        [ethers.constants.HashZero]
      );
      await expect(tx).to.be.revertedWith(
        "Output is not contained in the final hash"
      );

      tx = descartes
        .connect(claimer)
        .submitClaim(
          0,
          ethers.constants.HashZero,
          [[ethers.constants.HashZero]],
          ethers.constants.HashZero,
          [ethers.constants.HashZero]
        );

      await expect(tx)
        .to.emit(descartes, "ClaimSubmitted")
        .withArgs(0, ethers.constants.HashZero);

      const tx2 = await descartes.getState(0, mainSignerAddress);
      expect(tx2[0][2]).to.equal(outputPosition);
      expect(tx2).to.include.deep.members([
        [
          challengerAddress, // @TODO order inconsistency
          claimerAddress,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.constants.HashZero, // claimedOutput
          ethers.utils.formatBytes32String("WaitingConfirmation"), // currentState
        ],
      ]);
    });

    it("Should fail to abortByDeadline", async () => {
      let tx = descartes.abortByDeadline(0);
      await expect(tx).to.be.revertedWith(
        "Deadline is not over for this specific state"
      );

      const revertSnapshot = await takeSnapshot();
      await advanceTime(mainSigner.provider, finalTime);

      tx = descartes.abortByDeadline(0);
      await expect(tx).to.be.revertedWith("Cannot abort current state");
      await revertSnapshot();
    });

    it("Should confirm", async () => {
      const revertSnapshot = await takeSnapshot();
      let tx = descartes.confirm(0);
      await expect(tx).to.be.revertedWith("Cannot be called by user");

      tx = descartes.connect(challenger).confirm(0);
      await expect(tx).to.emit(descartes, "ResultConfirmed").withArgs(0);

      const tx2 = await descartes.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("ConsensusResult")
      );

      const tx3 = await descartes.getResult(0);
      expect(tx3).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = Object.values(tx3);
      expect(resultReady).to.be.true;
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal(ethers.constants.HashZero);

      await revertSnapshot();
    });

    it("Should get empty getSubInstances", async () => {
      let tx = await descartes.getSubInstances(0, mainSignerAddress);
      expect(tx).to.have.length(2);
      expect(tx._addresses).to.be.empty;
      expect(tx._indices).to.be.empty;
    });
    it("Should challenge", async () => {
      let tx = descartes.challenge(0);
      await expect(tx).to.be.revertedWith("Cannot be called by user");

      await mockVG.mock.instantiate.returns(123);
      tx = descartes.connect(challenger).challenge(0);
      await expect(tx).to.emit(descartes, "ChallengeStarted").withArgs(0);

      const tx2 = await descartes.getCurrentState(0);
      expect(tx2).to.be.equal(
        ethers.utils.formatBytes32String("WaitingChallenge")
      );

      const getMaxInstanceDuration = 222;
      await mockVG.mock.getMaxInstanceDuration.returns(getMaxInstanceDuration);
      const tx3 = await descartes.getState(0, mainSignerAddress);
      expect(tx3).to.have.length(4);
      expect(tx3[0]).to.have.length(3);
      expect(tx3[0][1]).to.be.equal(
        instantiateTimestamp + getMaxInstanceDuration
      );

      const tx4 = await descartes.getResult(0);
      expect(tx4).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = Object.values(tx4);
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.true;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal(ethers.constants.HashZero);
    });

    it("Should get vg at getSubInstances", async () => {
      let tx = await descartes.getSubInstances(0, mainSignerAddress);
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

      const tx = await descartes.winByVG(0);
      const tx2 = await descartes.getCurrentState(0);
      expect(tx2).to.be.equal(ethers.utils.formatBytes32String("ChallengerWon"));
      const tx3 = await descartes.getResult(0);
      expect(tx3).to.have.length(4);
      let [resultReady, sdkRunning, blameUser, result] = Object.values(tx3);
      expect(resultReady).to.be.false; // @discuss should it really be false?
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(challengerAddress);
      expect(result).to.be.equal(ethers.constants.HashZero);
      await revertSnapshot();

      // ---- Claimer Wins
      revertSnapshot = await takeSnapshot();
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(true);

      await descartes.winByVG(0);
      const tx4 = await descartes.getCurrentState(0);
      expect(tx4).to.be.equal(ethers.utils.formatBytes32String("ClaimerWon"));
      const tx5 = await descartes.getResult(0);
      expect(tx5).to.have.length(4);
      [resultReady, sdkRunning, blameUser, result] = Object.values(tx5);
      expect(resultReady).to.be.false; // @discuss should it really be false?
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(claimerAddress);
      expect(result).to.be.equal(ethers.constants.HashZero);

      const tx6 = await descartes.getState(0, mainSignerAddress);
      expect(tx6).to.have.length(4);
      expect(tx6[0][1]).to.equal(instantiateTimestamp + 0); // lastMoveTime  = now + 

      await revertSnapshot();

      // ---- VG is not finished
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(false);
      await expect(descartes.winByVG(0)).to.be.revertedWith("State of VG is not final");
    });
  });

  describe("Descartes with Providing steps", () => {
    let descartesIdx = 1;
    it("Should instantiate with different types of drives", async () => {
      const drives = [
        { ...aDrive, waitsProvider: true },
        { ...aDrive, needsLogger: true, waitsProvider: true },
        { ...aDrive, needsLogger: true, waitsProvider: false },
      ];
      await mockLogger.mock.isLogAvailable.returns(true);
      const tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimerAddress,
        challengerAddress,
        drives
      );
      const transaction = await tx;
      const txResult = await transaction.wait();
      descartesIdx = ethers.BigNumber.from(txResult.logs[0].data).toNumber();
      await expect(tx)
        .to.emit(descartes, "DescartesCreated")
        .withArgs(descartesIdx);

      // save 'now' used in other pieces of the contract
      const timestamp = await getBlockTimestampByHash(
        mainSigner.provider,
        transaction.blockHash
      );
      instantiateTimestamp = timestamp;

      const tx2 = await descartes.getState(descartesIdx, mainSignerAddress);
      expect(tx2[0][0]).to.equal(finalTime);
      // lastMoveTime  = now + timeToStartMachine(40) + maxLoggerUploadTime(40 * 60)
      expect(tx2[0][1]).to.equal(timestamp + 40 + 40 * 60);
      expect(tx2[0][2]).to.equal(outputPosition);
      expect(tx2).to.include.deep.members([
        [
          challengerAddress, // @TODO order inconsistency
          claimerAddress,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.constants.HashZero, // claimedOutput
          ethers.utils.formatBytes32String("WaitingProviders"), // currentState
        ],
      ]);
      expect(tx2[3]).to.have.length(1);
      driveMatcher(tx2[3][0], drives[0]);
    });

    it("Should abortByDeadline - ProviderMissedDeadline", async () => {
      let revertSnapshot = await takeSnapshot();
      //timeToStartMachine(40) + maxLoggerUploadTime(40 * 60) + 1// so it's more than
      await advanceTime(mainSigner.provider, 41 + 40 * 60);

      await descartes.abortByDeadline(descartesIdx);
      const tx = await descartes.getCurrentState(descartesIdx);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("ProviderMissedDeadline")
      );
      const tx2 = await descartes.getResult(descartesIdx);
      expect(tx2).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = Object.values(tx2);
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.false; // @discuss isn't it stopped?
      expect(blameUser).to.be.equal(aDrive.provider); // claimer
      expect(result).to.be.equal(ethers.constants.HashZero);
      await revertSnapshot();
    });

    it("Should claim(Direct/Logger)Drive correctly", async () => {
      let data = "0x12";
      let tx = descartes.claimDirectDrive(descartesIdx, data);
      await expect(tx).to.be.revertedWith("The sender is not provider");
      tx = descartes.connect(claimer).claimDirectDrive(descartesIdx, data);
      await expect(tx).to.be.revertedWith(
        "driveValue doesn't match driveLog2Size"
      );

      data = "0x" + "12".repeat(8);
      tx = descartes.connect(claimer).claimDirectDrive(descartesIdx, data);
      await expect(tx).to.not.be.reverted;

      tx = descartes.connect(claimer).claimDirectDrive(descartesIdx, data);
      await expect(tx).to.be.revertedWith(
        "Invalid drive to claim for direct value"
      );

      data = "0x" + "12".repeat(32);
      tx = descartes.connect(claimer).claimLoggerDrive(descartesIdx, data);
      await expect(tx).to.not.be.reverted;

      expect(await descartes.getCurrentState(descartesIdx))
        .to.be.equal(ethers.utils.formatBytes32String("WaitingClaim"));
    });

    it('Should call claimDirectDrive and transition to WaitingClaim', async () => {
      const drives = [
        { ...aDrive, waitsProvider: true },
      ];
      const data = "0x" + "12".repeat(8);
      const tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimerAddress,
        challengerAddress,
        drives
      );
      const txResult = await (await tx).wait();
      descartesIdx = ethers.BigNumber.from(txResult.logs[0].data).toNumber();
      const tx2 = descartes.connect(claimer).claimDirectDrive(descartesIdx, data);
      await expect(tx2).to.not.be.reverted;

      expect(await descartes.getCurrentState(descartesIdx))
        .to.be.equal(ethers.utils.formatBytes32String("WaitingClaim"));
    })
  });
});
