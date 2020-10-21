// Copyright (C) 2020 Cartesi Pte. Ltd.

// SPDX-License-Identifier: GPL-3.0-only
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
// PARTICULAR PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the GNU
// GPL, version 3, and so you should treat this component as a whole as being under
// the GPL version 3. But all Cartesi-written code in this component is licensed
// under the Apache License, version 2, or a compatible permissive license, and can
// be used independently under the Apache v2 license. After this component is
// rewritten, the entire component will be released under the Apache v2 license.

/// @title Descartes
/// @author Stephen Chen
pragma solidity ^0.7.0;
pragma experimental ABIEncoderV2;

// #if BUILD_TEST
import "./test/TestMerkle.sol";
// #else
import "@cartesi/util/contracts/Merkle.sol";
// #endif
import "@cartesi/util/contracts/Decorated.sol";
import "@cartesi/util/contracts/InstantiatorImpl.sol";
import "@cartesi/logger/contracts/LoggerInterface.sol";
import "@cartesi/arbitration/contracts/VGInterface.sol";
import "./DescartesInterface.sol";


contract Descartes is InstantiatorImpl, Decorated, DescartesInterface {
    address machine; // machine which will run the challenge
    LoggerInterface li;
    VGInterface vg;

    struct DescartesCtx {
        address owner; // the one who has power to shutdown the instance
        uint256 revealDrivesPointer; // the pointer to the current reveal drive
        uint256 providerDrivesPointer; // the pointer to the current provider drive
        uint256 finalTime; // max number of machine cycle to run
        uint64 outputPosition; // memory position of machine output
        uint8 outputLog2Size; // log2 size of the output drive in the unit of bytes
        uint256 roundDuration; // time interval to interact with this contract
        uint256 timeOfLastMove; // last time someone made a move with deadline
        uint256 vgInstance;
        bytes32 templateHash; // pristine hash of machine
        bytes32 initialHash; // initial hash with all drives mounted
        bytes32 claimedFinalHash; // claimed final hash of the machine
        bytes claimedOutput; // claimed final machine output
        address claimer; // responsible for claiming the machine output
        address currentChallenger; // it tracks who did the last challenge
        address[] partiesArray; // user can challenge claimer's output
        uint64 votesCounter;  // helps manage end state
        mapping(address => Party) parties; // control structure for challengers
        State currentState;
        uint256[] revealDrives; // indices of the reveal drives
        uint256[] providerDrives; // indices of the provider drives
        bytes32[] driveHash; // root hash of the drives
        Drive[] inputDrives;
    }

    mapping(uint256 => DescartesCtx) internal instance;

    // These are the possible states and transitions of the contract.

    // +---+
    // |   |
    // +---+
    //   |
    //   | instantiate
    //   v
    // +------------------+    abortByDeadline    +------------------------+
    // | WaitingProviders |---------------------->| ProviderMissedDeadline |
    // +------------------+                       +------------------------+
    //   |
    //   | provideLoggerDrive
    //   | or
    //   | provideDirectDrive
    //   v
    // +----------------+   abortByDeadline    +------------------------+
    // | WaitingReveals |--------------------->| ProviderMissedDeadline |
    // +----------------+                      +------------------------+
    //   |
    //   | revealLoggerDrive
    //   v
    // +--------------+   abortByDeadline    +-----------------------+
    // | WaitingClaim |--------------------->| ClaimerMissedDeadline |
    // +--------------+                      +-----------------------+
    //   |
    //   |
    //   |
    //   | submitClaim
    //   v
    // +-----------------------------+             +-----------------+
    // | WaitingConfirmationDeadline |------------>| ConsensusResult |
    // +----------------------------+   deadline  +-----------------+
    //   |
    //   |
    //   | challenge
    //   v
    // +------------------------+    winByVG     +------------+  if there are challengers
    // | WaitingChallengeResult |--------------->| ClaimerWon |-----------------------> WaitingConfirmationDeadline
    // +-----------------------+                +------------+    left; go back to
    //   |
    //   |
    //   |                  winByVG        +---------------+  if there are challengers
    //   +-------------------------------->| ChallengerWon |------------------------> WaitingClaim
    //                                     +---------------+  left; go back to
    //

    event DescartesCreated(
        uint256 _index
    );
    event ClaimSubmitted(uint256 _index, bytes32 _claimedFinalHash);
    event ResultConfirmed(uint256 _index);
    event ChallengeStarted(uint256 _index);
    event DescartesFinished(uint256 _index, uint8 _state);
    event DriveInserted(uint256 _index, Drive _drive);

    constructor(
        address _liAddress,
        address _vgAddress,
        address _machineAddress)
    {
        machine = _machineAddress;
        vg = VGInterface(_vgAddress);
        li = LoggerInterface(_liAddress);
    }

    /// @notice Instantiate a Descartes SDK instance.
    /// @param _finalTime max cycle of the machine for that computation
    /// @param _templateHash hash of the machine with all drives empty
    /// @param _outputPosition position of the output drive
    /// @param _roundDuration duration of the round (security param)
    /// @param _inputDrives an array of drive which assemble the machine
    /// @return uint256, Descartes index
    function instantiate(
        uint256 _finalTime,
        bytes32 _templateHash,
        uint64 _outputPosition,
        uint8 _outputLog2Size,
        uint256 _roundDuration,
        address[] memory parties,
        Drive[] memory _inputDrives) public returns (uint256)
    {
        DescartesCtx storage i = instance[currentIndex];

        for(uint256 j = 0; j < parties.length; j++) {
            require(i.parties[parties[j]].isParty == false, "Repetition of parties' addresses is not allowed");
            i.parties[parties[j]].isParty = true;
            i.partiesArray.push(parties[j]);
        }


        bool needsProviderPhase = false;
        uint256 drivesLength = _inputDrives.length;
        i.driveHash = new bytes32[](drivesLength);
        for (uint256 j = 0; j < drivesLength; j++) {
            Drive memory drive = _inputDrives[j];

            if (!drive.needsLogger) {
                require(drive.driveLog2Size >= 3, "directValue has to be at least one word");
                require(drive.driveLog2Size <= 10, "directValue cannot be bigger than 1kB");

                if (!drive.waitsProvider) {
                    require(
                        drive.directValue.length <= 2 ** drive.driveLog2Size,
                        "Input bytes length exceeds the claimed log2 size"
                    );

                    // pad zero to the directValue if it's not exact power of 2
                    bytes memory paddedDirectValue = drive.directValue;
                    if (drive.directValue.length < 2 ** drive.driveLog2Size) {
                        paddedDirectValue = abi.encodePacked(
                                drive.directValue,
                                new bytes(2 ** drive.driveLog2Size - drive.directValue.length)
                        );
                    }

                    bytes32[] memory data = getWordHashesFromBytes(paddedDirectValue);
                    i.driveHash[j] = Merkle.calculateRootFromPowerOfTwo(data);
                } else {
                    needsProviderPhase = true;
                    i.providerDrives.push(j);
                }
            } else {
                if (!drive.waitsProvider) {
                    i.driveHash[j] = drive.loggerRootHash;
                    if (!li.isLogAvailable(drive.loggerRootHash, drive.driveLog2Size)) {
                        i.revealDrives.push(j);
                    }
                } else {
                    needsProviderPhase = true;
                    i.providerDrives.push(j);
                }
            }
            i.inputDrives.push(Drive(
                drive.position,
                drive.driveLog2Size,
                drive.directValue,
                drive.loggerIpfsPath,
                drive.loggerRootHash,
                drive.provider,
                drive.waitsProvider,
                drive.needsLogger
            ));
        }

        require(_outputLog2Size >= 3, "output drive has to be at least one word");

        i.owner = msg.sender;
        i.claimer = parties[0]; // first on the list is selected to be claimer
        i.votesCounter = 1;  // first vote is always a submitClaim, so we count it once here
        i.finalTime = _finalTime;
        i.templateHash = _templateHash;
        i.initialHash = _templateHash;
        i.outputPosition = _outputPosition;
        i.outputLog2Size = _outputLog2Size;
        i.roundDuration = _roundDuration;
        i.timeOfLastMove = block.timestamp;
        if (needsProviderPhase) {
            i.currentState = State.WaitingProviders;
        } else if (i.revealDrives.length > 0) {
            i.currentState = State.WaitingChallengeDrives;
        } else {
            i.currentState = State.WaitingClaim;
        }

        emit DescartesCreated(
            currentIndex
        );
        active[currentIndex] = true;
        return currentIndex++;
    }


    /// @notice Challenger disputes the claim, starting a verification game.
    /// @param _index index of Descartes instance which challenger is starting the VG.
    function challenge(uint256 _index) public
        onlyInstantiated(_index)
        onlyByParty(_index)
        onlyNoVotes(_index)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(i.currentState == State.WaitingConfirmationDeadline, "State should be WaitingConfirmationDeadline");

        i.vgInstance = vg.instantiate(
            msg.sender, // challenger
            i.claimer,
            i.roundDuration,
            machine,
            i.initialHash,
            i.claimedFinalHash,
            i.finalTime);
        i.currentState = State.WaitingChallengeResult;
        i.parties[msg.sender].hasVoted = true;
        i.currentChallenger = msg.sender;
        i.votesCounter++;
        i.timeOfLastMove = block.timestamp;

        // @dev should we update timeOfLastMove over here too?
        emit ChallengeStarted(_index);
    }

    /// @notice User requesting content of all drives to be revealed.
    /// @param _index index of Descartes instance which is requested for the drives
    function challengeDrives(uint256 _index) public
        onlyInstantiated(_index)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(
            i.currentState == State.WaitingChallengeDrives,
            "State should be WaitingChallengeDrives"
        );
        require(i.parties[msg.sender].isParty, "Only concerned users can challengDrives");

        i.currentState = State.WaitingReveals;
        i.timeOfLastMove = block.timestamp;

    }

    /// @notice Claimer claims the machine final hash and also validate the drives and initial hash of the machine.
    /// @param _claimedFinalHash is the final hash of the machine
    /// @param _drivesSiblings is an array of siblings of each drive (see below example)
    /// @param _output is the bytes32 value of the output position
    /// @param _outputSiblings is the siblings of the output drive
    /// @dev Example: consider 3 drives, the first drive's siblings should be a pristine machine.
    ///      The second drive's siblings should be the machine with drive 1 mounted.
    ///      The third drive's siblings should be the machine with drive 2 mounted.
    function submitClaim(
        uint256 _index,
        bytes32 _claimedFinalHash,
        bytes32[][] memory _drivesSiblings,
        bytes memory _output,
        bytes32[] memory _outputSiblings) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].claimer)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        bool deadlinePassed = block.timestamp > i.timeOfLastMove + getMaxStateDuration(_index);
        require(
            i.currentState == State.WaitingClaim ||
            (i.currentState == State.WaitingChallengeDrives && deadlinePassed),
            "State should be WaitingClaim, or WaitingChallengeDrives with deadline passed");
        require(i.inputDrives.length == _drivesSiblings.length, "Claimed drive number should match claimed siblings number");
        require(_output.length == 2 ** i.outputLog2Size, "Output length doesn't match output log2 size");

        bytes32[] memory data = getWordHashesFromBytes(_output);
        require(
            Merkle.getRootWithDrive(
                i.outputPosition,
                i.outputLog2Size,
                Merkle.calculateRootFromPowerOfTwo(data),
                _outputSiblings) == _claimedFinalHash,
            "Output is not contained in the final hash"
        );

        uint256 drivesLength = i.inputDrives.length;
        for (uint256 j = 0; j < drivesLength; j++) {
            bytes32[] memory driveSiblings = _drivesSiblings[j];
            require(
                Merkle.getRootWithDrive(
                    i.inputDrives[j].position,
                    i.inputDrives[j].driveLog2Size,
                    Merkle.getPristineHash(uint8(i.inputDrives[j].driveLog2Size)),
                    driveSiblings) == i.initialHash,
                "Drive siblings must be compatible with previous initial hash for empty drive"
            );
            i.initialHash = Merkle.getRootWithDrive(
                i.inputDrives[j].position,
                i.inputDrives[j].driveLog2Size,
                i.driveHash[j],
                driveSiblings
            );
        }

        i.claimedFinalHash = _claimedFinalHash;
        i.currentState = State.WaitingConfirmationDeadline;
        i.claimedOutput = _output;
        i.parties[i.claimer].hasVoted = true;
        i.timeOfLastMove = block.timestamp;


        emit ClaimSubmitted(_index, _claimedFinalHash);
    }

    /// @notice Is the given user concern about this instance.
    function isConcerned(uint256 _index, address _user)
        public
        override
        view
        onlyInstantiated(_index)
        returns (bool)
    {
        DescartesCtx storage i = instance[_index];
        return i.parties[_user].isParty;
    }

    function getPartyState(uint256 _index, address _p) public view
        onlyInstantiated(_index)
        returns (
            bool isParty,
            bool hasVoted,
            bool hasCheated
        )
    {
        Party storage party = instance[_index].parties[_p];
        isParty = party.isParty;
        hasVoted = party.hasVoted;
        hasCheated = party.hasCheated;
    }
    /// @notice Get state of the instance concerning given user.
    function getState(uint256 _index, address _user) public view
        onlyInstantiated(_index)
        returns (
            uint256[] memory,
            address[] memory,
            bytes32[] memory,
            bytes memory,
            Drive[] memory,
            Party memory user
        )
    {
        DescartesCtx storage i = instance[_index];

        user = i.parties[_user];

        uint256[] memory uintValues = new uint256[](4);
        uintValues[0] = i.finalTime;
        uintValues[1] = i.timeOfLastMove + getMaxStateDuration(
            _index);
        uintValues[2] = i.outputPosition;
        uintValues[3] = i.outputLog2Size;

        address[] memory addressValues = new address[](2);
        addressValues[0] = i.currentChallenger;
        addressValues[1] = i.claimer;

        bytes32[] memory bytes32Values = new bytes32[](4);
        bytes32Values[0] = i.templateHash;
        bytes32Values[1] = i.initialHash;
        bytes32Values[2] = i.claimedFinalHash;
        bytes32Values[3] = getCurrentState(_index);

        if (i.currentState == State.WaitingProviders) {
            Drive[] memory drives = new Drive[](1);
            drives[0] = i.inputDrives[i.providerDrives[i.providerDrivesPointer]];
            return (
                uintValues,
                addressValues,
                bytes32Values,
                i.claimedOutput,
                drives,
                user
            );
        } else if (i.currentState == State.WaitingReveals) {
            Drive[] memory drives = new Drive[](1);
            drives[0] = i.inputDrives[i.revealDrives[i.revealDrivesPointer]];
            return (
                uintValues,
                addressValues,
                bytes32Values,
                i.claimedOutput,
                drives,
                user
            );
        } else if (i.currentState == State.ProviderMissedDeadline) {
            Drive[] memory drives = new Drive[](0);
            return (
                uintValues,
                addressValues,
                bytes32Values,
                i.claimedOutput,
                drives,
                user
            );
        } else {
            return (
                uintValues,
                addressValues,
                bytes32Values,
                i.claimedOutput,
                i.inputDrives,
                user
            );
        }
    }

    function getCurrentState(uint256 _index) public view
        onlyInstantiated(_index)
        returns (bytes32)
    {
        State currentState = instance[_index].currentState;
        if (currentState == State.WaitingProviders) {
            return "WaitingProviders";
        }
        if (currentState == State.WaitingReveals) {
            return "WaitingReveals";
        }
        if (currentState == State.WaitingChallengeDrives) {
            return "WaitingChallengeDrives";
        }
        if (currentState == State.ClaimerMissedDeadline) {
            return "ClaimerMissedDeadline";
        }
        if (currentState == State.ProviderMissedDeadline) {
            return "ProviderMissedDeadline";
        }
        if (currentState == State.WaitingClaim) {
            return "WaitingClaim";
        }
        if (currentState == State.WaitingConfirmationDeadline) {
            return "WaitingConfirmationDeadline";
        }
        if (currentState == State.WaitingChallengeResult) {
            return "WaitingChallengeResult";
        }
        if (currentState == State.ConsensusResult) {
            return "ConsensusResult";
        }
        if (currentState == State.ChallengerWon) {
            return "ChallengerWon";
        }
        if (currentState == State.ClaimerWon) {
            return "ClaimerWon";
        }

        revert("Unrecognized state");
    }

    /// @notice Get sub-instances of the instance.
    function getSubInstances(uint256 _index, address)
        public
        override
        view
        onlyInstantiated(_index)
        returns (address[] memory _addresses, uint256[] memory _indices)
    {
        address[] memory a;
        uint256[] memory i;

        if (instance[_index].currentState == State.WaitingChallengeResult) {
            a = new address[](1);
            i = new uint256[](1);
            a[0] = address(vg);
            i[0] = instance[_index].vgInstance;
        } else {
            a = new address[](0);
            i = new uint256[](0);
        }
        return (a, i);
    }

    /// @notice Provide the content of a direct drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to.
    /// @param _value bytes value of the direct drive
    function provideDirectDrive(uint256 _index, bytes memory _value) public
        onlyInstantiated(_index)
        requirementsForProviderDrive(_index)
    {
        DescartesCtx storage i = instance[_index];
        uint256 driveIndex = i.providerDrives[i.providerDrivesPointer];
        Drive storage drive = i.inputDrives[driveIndex];

        require(!drive.needsLogger, "Invalid drive to claim for direct value");
        require(
            _value.length <= 2 ** drive.driveLog2Size,
            "Input bytes length exceeds the claimed log2 size"
        );

        // pad zero to the directValue if it's not exact power of 2
        bytes memory paddedDirectValue = _value;
        if (_value.length < 2 ** drive.driveLog2Size) {
            paddedDirectValue = abi.encodePacked(
                    _value,
                    new bytes(2 ** drive.driveLog2Size - _value.length)
            );
        }

        bytes32[] memory data = getWordHashesFromBytes(paddedDirectValue);
        bytes32 driveHash = Merkle.calculateRootFromPowerOfTwo(data);

        drive.directValue = _value;
        i.driveHash[driveIndex] = driveHash;
        i.providerDrivesPointer++;
        i.timeOfLastMove = block.timestamp;

        if (i.providerDrivesPointer == i.providerDrives.length) {
            if (i.revealDrives.length > 0) {
                i.currentState = State.WaitingChallengeDrives;
            } else {
                i.currentState = State.WaitingClaim;
            }
        }

        emit DriveInserted(_index, i.inputDrives[driveIndex]);
    }

    /// @notice Provide the root hash of a logger drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to
    /// @param _root root hash of the logger drive
    function provideLoggerDrive(uint256 _index, bytes32 _root) public
        onlyInstantiated(_index)
        requirementsForProviderDrive(_index)
    {
        DescartesCtx storage i = instance[_index];
        uint256 driveIndex = i.providerDrives[i.providerDrivesPointer];
        Drive storage drive = i.inputDrives[driveIndex];

        require(drive.needsLogger, "Invalid drive to claim for logger");

        drive.loggerRootHash = _root;
        i.driveHash[driveIndex] = drive.loggerRootHash;
        i.providerDrivesPointer++;
        i.timeOfLastMove = block.timestamp;

        if (i.providerDrivesPointer == i.providerDrives.length) {
            if (i.revealDrives.length > 0) {
                i.currentState = State.WaitingChallengeDrives;
            } else {
                i.currentState = State.WaitingClaim;
            }
        }

        emit DriveInserted(_index, i.inputDrives[driveIndex]);
    }

    /// @notice Reveal the content of a logger drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to
    function revealLoggerDrive(uint256 _index) public
        onlyInstantiated(_index)
    {
        DescartesCtx storage i = instance[_index];
        uint256 driveIndex = i.revealDrives[i.revealDrivesPointer];
        require(i.currentState == State.WaitingReveals, "The state is not WaitingReveals");
        require(driveIndex < i.inputDrives.length, "Invalid driveIndex");

        Drive memory drive = i.inputDrives[driveIndex];

        require(drive.needsLogger, "needsLogger should be true");
        require(li.isLogAvailable(drive.loggerRootHash, drive.driveLog2Size), "Hash is not available on logger yet");

        i.revealDrivesPointer++;
        i.timeOfLastMove = block.timestamp;

        if (i.revealDrivesPointer == i.revealDrives.length) {
            i.currentState = State.WaitingClaim;
        }
    }

    /// @notice In case one of the parties wins the verification game,
    ///         then he or she can call this function to claim victory in
    ///         this contract as well.
    /// @param _index index of Descartes instance to win
    function winByVG(uint256 _index) public
        onlyInstantiated(_index)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(
            i.currentState == State.WaitingChallengeResult,
            "State is not WaitingChallengeResult, cannot winByVG"
        );
        i.timeOfLastMove = block.timestamp;
        uint256 vgIndex = i.vgInstance;

        if (vg.stateIsFinishedChallengerWon(vgIndex)) {
            if(i.votesCounter == i.partiesArray.length) {
                i.currentState = State.ChallengerWon;  
                return;
            }
            i.currentState = State.WaitingClaim;
            i.parties[i.claimer].hasCheated = true;
            i.claimer = i.currentChallenger;
            i.currentChallenger = address(0);
            return;
        }

        if (vg.stateIsFinishedClaimerWon(vgIndex)) {
            if(i.votesCounter == i.partiesArray.length) {
                i.currentState = State.ClaimerWon;
                return;
            }
            i.currentState = State.WaitingConfirmationDeadline;
            i.parties[i.currentChallenger].hasCheated = true;
            i.currentChallenger = address(0);
            return;
        }
        require(false, "State of VG is not final");
    }

    /// @notice Deactivate a Descartes SDK instance.
    /// @param _index index of Descartes instance to deactivate
    function destruct(uint256 _index)
        public
        override     
        onlyInstantiated(_index)
        onlyBy(instance[_index].owner)
    {
        DescartesCtx storage i = instance[_index];
        require(
            i.currentState == State.ProviderMissedDeadline ||
            i.currentState == State.ClaimerMissedDeadline ||
            i.currentState == State.ConsensusResult ||
            i.currentState == State.ChallengerWon ||
            i.currentState == State.ClaimerWon,
            "Cannot destruct instance at current state"
        );

        delete i.revealDrives;
        delete i.providerDrives;
        delete i.driveHash;
        delete i.inputDrives;
        deactivate(_index);
    }

    /// @notice Abort the instance by missing deadline.
    /// @param _index index of Descartes instance to abort
    function abortByDeadline(uint256 _index) public onlyInstantiated(_index) {
        DescartesCtx storage i = instance[_index];
        bool afterDeadline = block.timestamp > (
            i.timeOfLastMove + getMaxStateDuration(
                _index
            )
        );

        require(afterDeadline, "Deadline is not over for this specific state");

        if (i.currentState == State.WaitingProviders) {
            i.currentState = State.ProviderMissedDeadline;
            return;
        }
        if (i.currentState == State.WaitingReveals) {
            i.currentState = State.ProviderMissedDeadline;
            return;
        }
        if (i.currentState == State.WaitingClaim) {
            i.currentState = State.ClaimerMissedDeadline;
            return;
        }
        if (i.currentState == State.WaitingConfirmationDeadline) {
            i.currentState = State.ConsensusResult;
            return;
        }

        revert("Cannot abort current state");
    }

    /// @notice Get result of a finished instance.
    /// @param _index index of Descartes instance to get result
    /// @return bool, indicates the result is ready
    /// @return bool, indicates the sdk is still running
    /// @return address, the user to blame for the abnormal stop of the sdk
    /// @return bytes, the result of the sdk if available
    function getResult(uint256 _index)
        public
        override
        view
        onlyInstantiated(_index)
        returns (bool, bool, address, bytes memory)
    {
        DescartesCtx storage i = instance[_index];
        if (i.currentState == State.ConsensusResult) {
            return (true, false, address(0), i.claimedOutput);
        }
        if (i.currentState == State.WaitingProviders ||
            i.currentState == State.WaitingClaim ||
            i.currentState == State.WaitingConfirmationDeadline ||
            i.currentState == State.WaitingChallengeResult) {
            return (false, true, address(0), "");
        }
        if (i.currentState == State.ProviderMissedDeadline) {
            address userToBlame = address(0);
            // check if resulted from the WaitingProviders phase
            if (instance[_index].providerDrivesPointer < instance[_index].providerDrives.length) {
                userToBlame = i.inputDrives[i.providerDrives[i.providerDrivesPointer]].provider;
            // check if resulted from the WaitingReveals phase
            } else if (instance[_index].revealDrivesPointer < instance[_index].revealDrives.length) {
                userToBlame = i.inputDrives[i.revealDrives[i.revealDrivesPointer]].provider;
            }
            return (false, false, userToBlame, "");
        }
        if (i.currentState == State.ClaimerMissedDeadline ||
            i.currentState == State.ChallengerWon) {
            return (false, false, i.claimer, "");
        }
        if (i.currentState == State.ClaimerWon) {
            return (false, false, i.currentChallenger, "");
        }

        revert("Unrecognized state");
    }

    /// @notice Convert bytes32 into bytes8[] and calculate the hashes of them
    function getWordHashesFromBytes32(bytes32 _value) private pure returns(bytes32[] memory) {
        bytes32[] memory data = new bytes32[](4);
        for (uint256 i = 0; i < 4; i++) {
            bytes8 dataBytes8 = bytes8(_value << (i * 64) & 0xffffffffffffffff000000000000000000000000000000000000000000000000);
            data[i] = keccak256(abi.encodePacked(dataBytes8));
        }
        return data;
    }

    /// @notice Convert bytes into bytes8[] and calculate the hashes of them
    function getWordHashesFromBytes(bytes memory _value) private pure returns(bytes32[] memory) {
        uint256 hashesLength = _value.length/8;
        bytes32[] memory data = new bytes32[](hashesLength);
        for (uint256 i = 0; i < hashesLength; i++) {
            bytes8 dataBytes8;
            for (uint256 j = 0; j < 8; j++) {
                bytes8 tempBytes8 = _value[i * 8 + j];
                tempBytes8 = tempBytes8 >> (j * 8);
                dataBytes8 = dataBytes8 | tempBytes8;
            }
            data[i] = keccak256(abi.encodePacked(dataBytes8));
        }
        return data;
    }

    /// @notice Get the worst case scenario duration for a specific state
    function getMaxStateDuration(
        uint256 _index
    ) private view returns (uint256)
    {
        // TODO: make sure maxDuration calculations are reasonable
        uint256 partitionSize = 1;
        uint256 picoSecondsToRunInsn = 500; // 500 pico seconds to run a instruction
        uint256 timeToStartMachine = 40; // 40 seconds to start the machine for the first time

        if (instance[_index].currentState == State.WaitingProviders) {
            // time to react
            return instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.WaitingReveals) {
            // time to upload to logger + time to react
            uint256 maxLoggerUploadTime = 40 * 60;
            return maxLoggerUploadTime +
                instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.WaitingChallengeDrives) {
            // number of logger drives * time to react
            return instance[_index].revealDrives.length * 2 *
                instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.WaitingClaim) {
            // time to run entire machine + time to react
            return timeToStartMachine +
                ((instance[_index].finalTime * picoSecondsToRunInsn) / 1e12) +
                instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.WaitingConfirmationDeadline) {
            // time to run entire machine + time to react
            return timeToStartMachine +
                ((instance[_index].finalTime * picoSecondsToRunInsn) / 1e12) +
                instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.WaitingChallengeResult) {
            // time to run a verification game + time to react
            return vg.getMaxInstanceDuration(
                instance[_index].roundDuration,
                timeToStartMachine,
                partitionSize,
                instance[_index].finalTime,
                picoSecondsToRunInsn) + instance[_index].roundDuration;
        }

        if (instance[_index].currentState == State.ClaimerWon ||
            instance[_index].currentState == State.ChallengerWon ||
            instance[_index].currentState == State.ClaimerMissedDeadline ||
            instance[_index].currentState == State.ConsensusResult) {
            return 0; // final state
        }
    }

    /// @notice several require statements for a drive
    modifier requirementsForProviderDrive(uint256 _index) {
        DescartesCtx storage i = instance[_index];
        require(i.currentState == State.WaitingProviders, "The state is not WaitingProviders");
        require(i.providerDrivesPointer < i.providerDrives.length, "No available pending drives");

        uint256 driveIndex = i.providerDrives[i.providerDrivesPointer];
        require(driveIndex < i.inputDrives.length, "Invalid drive index");

        Drive memory drive = i.inputDrives[driveIndex];
        require(i.driveHash[driveIndex] == bytes32(0), "The drive hash shouldn't be filled");
        require(drive.waitsProvider, "waitProvider should be true");
        require(drive.provider == msg.sender, "The sender is not provider");

        _;
    }

    /// @notice checks whether or not it's a party to this instance
    modifier onlyByParty(uint _index) {
        DescartesCtx storage i = instance[_index];
        require(i.parties[msg.sender].isParty, "The sender is not party to this instance");
        _;
    }
    /// @notice checks whether or not it's a party to this instance
    modifier onlyNoVotes(uint _index) {
        DescartesCtx storage i = instance[_index];
        require(!i.parties[msg.sender].hasVoted, "Sender has already challenged or claimed");
        _;
    }
}
