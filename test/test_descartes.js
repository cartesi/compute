const { ethers } = require("@nomiclabs/buidler");
const { expect, use } = require("chai");
const { solidity } = require("ethereum-waffle");
const { deployMockContract } = require("@ethereum-waffle/mock-contract");
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

const deployDescartes = async ({ logger, vg, step }) => {
  const LoggerAddress = logger || LoggerJson.networks[network_id].address;
  const VGAddress = vg || VGInstantiatorJson.networks[network_id].address;
  const StepAddress = step || StepJson.networks[network_id].address;
  const Descartes = await ethers.getContractFactory("TestDescartes");
  const descartes = await Descartes.deploy(
    LoggerAddress,
    VGAddress,
    StepAddress
  );
  await descartes.deployed();
  return descartes;
};

describe("Descartes tests", () => {
  let mainSigner, claimer, challenger;
  let accounts;
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
  };
  let descartes;
  let takeSnapshot;
  let mockVG;

  before(async () => {
    accounts = await ethers.getSigners();
    [mainSigner, claimer, challenger] = accounts;
    aDrive.provider = claimer._address;
    takeSnapshot = snapshotTaker(mainSigner.provider);

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
      let tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimer._address,
        challenger._address,
        [aDrive]
      );
      await expect(tx).to.emit(descartes, "DescartesCreated").withArgs(0);
      // save 'now' used in other pieces of the contract
      const timestamp = await getBlockTimestampByHash(
        mainSigner.provider,
        (await tx).blockHash
      );
      descartes.deployTimestamp = timestamp;

      tx = await descartes.getState(0, mainSigner._address);
      expect(tx[0][0]).to.equal(finalTime);
      expect(tx[0][1]).to.equal(timestamp + 40); // lastMoveTime  = now + timeToStartMachine(40)
      expect(tx[0][2]).to.equal(outputPosition);
      expect(tx).to.include.deep.members([
        [
          challenger._address, // @TODO order inconsistency
          claimer._address,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.constants.HashZero, // claimedOutput
          ethers.utils.formatBytes32String("WaitingClaim"), // currentState
        ],
      ]);
      expect(tx[3]).to.have.length(1);
      driveMatcher(tx[3][0], aDrive);
    });

    it("Should respond isConcerned correctly", async () => {
      let res = await descartes.isConcerned(0, claimer._address);
      expect(res).to.equal(true);

      res = await descartes.isConcerned(0, challenger._address);
      expect(res).to.equal(true);

      res = await descartes.isConcerned(0, mainSigner._address);
      expect(res).to.equal(false);

      res = descartes.isConcerned(1, claimer._address);
      await expect(res).to.be.revertedWith("Index not instantiated");
    });

    it("Should succeed to abortByDeadline -ClaimerMissedDeadline-", async () => {
      let revertSnapshot = await takeSnapshot();
      await advanceTime(mainSigner.provider, finalTime);
      let tx = await descartes.abortByDeadline(0);
      tx = await descartes.getCurrentState(0);
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

      tx = await descartes.getState(0, mainSigner._address);
      expect(tx[0][2]).to.equal(outputPosition);
      expect(tx).to.include.deep.members([
        [
          challenger._address, // @TODO order inconsistency
          claimer._address,
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

      tx = await descartes.getCurrentState(0);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("ConsensusResult")
      );

      tx = await descartes.getResult(0);
      expect(tx).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = tx;
      expect(resultReady).to.be.true;
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal(ethers.constants.HashZero);

      await revertSnapshot();
    });

    it("Should get empty getSubInstances", async () => {
      let tx = await descartes.getSubInstances(0, mainSigner._address);
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

      tx = await descartes.getCurrentState(0);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("WaitingChallenge")
      );

      const getMaxInstanceDuration = 222;
      await mockVG.mock.getMaxInstanceDuration.returns(getMaxInstanceDuration);
      tx = await descartes.getState(0, mainSigner._address);
      expect(tx).to.have.length(4);
      expect(tx[0]).to.have.length(3);
      expect(tx[0][1]).to.be.equal(
        descartes.deployTimestamp + getMaxInstanceDuration
      );

      tx = await descartes.getResult(0);
      expect(tx).to.have.length(4);
      const [resultReady, sdkRunning, blameUser, result] = tx;
      expect(resultReady).to.be.false;
      expect(sdkRunning).to.be.true;
      expect(blameUser).to.be.equal(ethers.constants.AddressZero);
      expect(result).to.be.equal(ethers.constants.HashZero);
    });

    it("Should get vg at getSubInstances", async () => {
      let tx = await descartes.getSubInstances(0, mainSigner._address);
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

      let tx = await descartes.winByVG(0);
      tx = await descartes.getCurrentState(0);
      expect(tx).to.be.equal(ethers.utils.formatBytes32String("ChallengerWon"));
      tx = await descartes.getResult(0);
      expect(tx).to.have.length(4);
      let [resultReady, sdkRunning, blameUser, result] = tx;
      expect(resultReady).to.be.false; // @discuss should it really be false?
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(challenger._address);
      expect(result).to.be.equal(ethers.constants.HashZero);
      await revertSnapshot();

      // ---- Claimer Wins
      revertSnapshot = await takeSnapshot();
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(true);

      tx = await descartes.winByVG(0);
      tx = await descartes.getCurrentState(0);
      expect(tx).to.be.equal(ethers.utils.formatBytes32String("ClaimerWon"));
      tx = await descartes.getResult(0);
      expect(tx).to.have.length(4);
      [resultReady, sdkRunning, blameUser, result] = tx;
      expect(resultReady).to.be.false; // @discuss should it really be false?
      expect(sdkRunning).to.be.false;
      expect(blameUser).to.be.equal(claimer._address);
      expect(result).to.be.equal(ethers.constants.HashZero);

      tx = await descartes.getState(0, mainSigner._address);
      expect(tx).to.have.length(4);
      expect(tx[0][1]).to.equal(descartes.deployTimestamp + 0); // lastMoveTime  = now + 

      await revertSnapshot();

      // ---- VG is not finished
      await mockVG.mock.stateIsFinishedChallengerWon.returns(false);
      await mockVG.mock.stateIsFinishedClaimerWon.returns(false);
      tx = descartes.winByVG(0);
      await expect(tx).to.be.revertedWith("State of VG is not final");
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
      let tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimer._address,
        challenger._address,
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
      descartes.deployTimestamp = timestamp;

      tx = await descartes.getState(descartesIdx, mainSigner._address);
      expect(tx[0][0]).to.equal(finalTime);
      // lastMoveTime  = now + timeToStartMachine(40) + maxLoggerUploadTime(40 * 60)
      expect(tx[0][1]).to.equal(timestamp + 40 + 40 * 60);
      expect(tx[0][2]).to.equal(outputPosition);
      expect(tx).to.include.deep.members([
        [
          challenger._address, // @TODO order inconsistency
          claimer._address,
        ],
        [
          templateHash,
          templateHash, // initialHash
          ethers.constants.HashZero, // claimedFinalHash
          ethers.constants.HashZero, // claimedOutput
          ethers.utils.formatBytes32String("WaitingProviders"), // currentState
        ],
      ]);
      expect(tx[3]).to.have.length(1);
      driveMatcher(tx[3][0], drives[0]);
    });

    it("Should abortByDeadline - ProviderMissedDeadline", async () => {
      let revertSnapshot = await takeSnapshot();
      //timeToStartMachine(40) + maxLoggerUploadTime(40 * 60) + 1// so it's more than
      await advanceTime(mainSigner.provider, 41 + 40 * 60);

      let tx = await descartes.abortByDeadline(descartesIdx);
      tx = await descartes.getCurrentState(descartesIdx);
      expect(tx).to.be.equal(
        ethers.utils.formatBytes32String("ProviderMissedDeadline")
      );
      tx = await descartes.getResult(descartesIdx);
      expect(tx).to.have.length(4);
      [resultReady, sdkRunning, blameUser, result] = tx;
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

      tx = await descartes.getCurrentState(descartesIdx);
      expect(tx).to.be.equal(ethers.utils.formatBytes32String("WaitingClaim"));
    });

    it('Should call claimDirectDrive and transition to WaitingClaim', async () => {
      const drives = [
        { ...aDrive, waitsProvider: true },
      ];
      const data = "0x" + "12".repeat(8);
      let tx = descartes.instantiate(
        finalTime,
        templateHash,
        outputPosition,
        roundDuration,
        claimer._address,
        challenger._address,
        drives
      );
      const txResult = await (await tx).wait();
      descartesIdx = ethers.BigNumber.from(txResult.logs[0].data).toNumber();
      tx = descartes.connect(claimer).claimDirectDrive(descartesIdx, data);
      await expect(tx).to.not.be.reverted;

      tx = await descartes.getCurrentState(descartesIdx);
      expect(tx).to.be.equal(ethers.utils.formatBytes32String("WaitingClaim"));
    })
  });
});
