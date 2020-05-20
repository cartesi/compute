// Copyright (C) 2020 Cartesi Pte. Ltd.

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
pragma solidity ^0.5.0;

import "@cartesi/util/contracts/Decorated.sol";
import "@cartesi/logger/contracts/LoggerInterface.sol";
import "@cartesi/arbitration/contracts/VGInterface.sol";
import "./DescartesInterface.sol";


contract Descartes is Decorated, DescartesInterface {
    address public owner;

    struct DecartesCtx {
        uint256 pendingDrivesPointer; // the pointer to the current pending drive
        uint256 finalTime; // max number of machine cycle to run
        uint256 outputPosition; // memory position of machine output
        uint256 roundDuration; // time interval to interact with this contract
        uint256 timeOfLastMove; // last time someone made a move with deadline
        uint256 vgInstance;
        bytes32 pristineHash; // pristine hash of machine
        bytes32 initialHash; // initial hash with all drives mounted
        bytes32 claimedFinalHash; // claimed final hash of the machine
        address claimer; // responsible for claiming the machine output
        address challenger; // user can challenge claimer's output
        address machine; // machine which will run the challenge
        LoggerInterface li;
        VGInterface vg;
        state currentState;
        uint256[] pendingDrives; // indices of the pending drives
        Drive[] drives;
    }

    mapping(uint256 => DecartesCtx) internal instance;

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
    //   | claimLoggerDrive
    //   | or 
    //   | claimIntegerDrive
    //   v
    // +--------------+   abortByDeadline    +-----------------------+
    // | WaitingClaim |--------------------->| ClaimerMisseddeadline |
    // +--------------+                      +-----------------------+
    //   |
    //   |
    //   |          
    //   | submitClaim
    //   v
    // +---------------------+  confirm    +-----------------+
    // | WaitingConfirmation |------------>| ConsensusResult |
    // +---------------------+ or deadline +-----------------+
    //   |
    //   |
    //   | challenge
    //   v
    // +------------------+    winByVG     +------------+
    // | WaitingChallenge |--------------->| ClaimerWon |
    // +------------------+                +------------+
    //   |
    //   |
    //   |                  winByVG        +---------------+
    //   +-------------------------------->| ChallengerWon |
    //                                     +---------------+
    //

    event ClaimSubmitted(uint256 _index, bytes32 _claimedFinalHash);
    event ResultConfirmed(uint256 _index);
    event ChallengeStarted(uint256 _index);
    event DescartesFinished(uint256 _index, uint8 _state);

    constructor() public {
        owner = msg.sender;
    }

    /// @notice Instantiate a Descartes SDK instance.
    /// @param _finalTime max cycle of the machine for that computation
    /// @param _pristineHash hash of the machine with all drives empty
    /// @param _outputPosition position of the output drive
    /// @param _roundDuration duration of the round (security param)
    /// @param _drives an array of drive which assemble the machine
    /// @return Descartes index
    function instantiate(
        uint256 _finalTime,
        bytes32 _pristineHash,
        uint256 _outputPosition,
        uint256 _roundDuration;
        address _claimer,
        address _challenger,
        address _liAddress,
        address _vgAddress,
        address _machineAddress,
        Drive[] _drives ) public returns (uint256)
        onlyBy(owner)
    {
        DescartesCtx storage i = instance[currentIndex];

        require(_challenger != _claimer, "Claimer cannot be a challenger");

        state memory currentState = state.WaitingClaim;
        uint256 drivesLength = _drives.length;
        for (uint256 j = 0; j < drivesLength; j++) {
            Drive memory drive = _drive[j];
            
            if (type == driveType.IntegerWithValue) {
                require(Merkle.getRootWithDrive(drive.position, drive.log2Size, Merkle.getPristineHash(uint8(drive.log2Size)), drive.siblings) == _pristineHash, "Drive siblings must be compatible with pristine hash for an empty drive");
                
                drive.ready = true;
                bytes8 memory[] data = convertUint256ToBytes8Array(drive.uintValue256);
                drive.driveHash = li.calculateMerkleRootFromData(2, data);
            } else if (type == driveType.LoggerWithHash) {
                if (li.isLogAvailable(drive.bytesValue32, drive.log2Size)) {
                    require(Merkle.getRootWithDrive(drive.position, drive.log2Size, Merkle.getPristineHash(uint8(drive.log2Size)), drive.siblings) == _pristineHash, "Drive siblings must be compatible with pristine hash for an empty drive");
                    
                    drive.ready = true;
                    drive.driveHash = drive.bytesValue32;
                } else {
                    drive.ready = false;
                    currentState = state.WaitingProviders;
                    i.pendingDrives.push(j);
                }
            } else if (drive.type == driveType.IntegerWithProvider || drive.type == driveType.LoggerWithProvider) {
                drive.ready = false;
                currentState = state.WaitingProviders;
                i.pendingDrives.push(j);
            } else {
                revert("Unknown Drive Type");
            }
        }
        
        i.challenger = _challenger;
        i.claimer = _claimer;
        i.machine = _machineAddress;
        i.vg = VGInterface(_vgAddress);
        i.li = LoggerInterface(_liAddress);
        i.finalTime = _finalTime;
        i.pristineHash = _pristineHash;
        i.initialHash = _pristineHash;
        i.outputPosition = _outputPosition;
        i.roundDuration = _roundDuration;
        i.timeOfLastMove = now;
        i.drives = _drives;
        i.currentState = currentState;

        active[currentIndex] = true;
        return currentIndex++;
    }

    /// @notice Challenger accepts claim.
    /// @param _index index of Descartes instance that the challenger is confirming the claim.
    function confirm(uint256 _index) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].challenger)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(i.currentState == state.WaitingConfirmation, "State should be WaitingConfirmation");
        
        i.currentState = state.ConsensusResult;
        emit ResultConfirmed(_index);
    }

    /// @notice Challenger disputes the claim, starting a verification game.
    /// @param _index index of Descartes instance which challenger is starting the VG.
    function challenge(uint256 _index) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].challenger)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(i.currentState == state.WaitingConfirmation, "State should be WaitingConfirmation");
        
        i.vgInstance = i.vg.instantiate(
            i.challenger,
            i.claimer,
            i.roundDuration,
            i.machine,
            i.initialHash,
            i.claimedFinalHash,
            i.finalTime);
        i.currentState = state.WaitingChallenge;

        emit ChallengeStarted(_index);
    }

    /// @notice Claimer claims the machine final hash and also validate the drives and initial hash of the machine.
    /// @param _drives are an array of ready drives, containing siblings of the drive which contains the prior drives in the order
    /// @dev Example: consider 3 drives, the first drive's siblings should be a pristine machine.
    ///      The second drive's siblings should be the machine with drive 1 mounted.
    ///      The third drive's siblings should be the machine with drive 2 mounted.
    function submitClaim(uint256 _index, bytes32 _claimedFinalHash, Drive[] _drives) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].claimer)
        increasesNonce(_index)
    {
        DescartesCtx storage i = instance[_index];
        require(i.currentState == state.WaitingClaim, "State should be WaitingClaim");
        require(_drives.length == i.drives.length);

        bytes32 initialHash = i.initialHash;
        uint256 drivesLength = _drives.length;
        for (uint256 j = 0; j < drivesLength; j++) {
            require(_drives[j].position == i.drives[j].position);
            require(_drives[j].log2Size == i.drives[j].log2Size);
            require(_drives[j].driveHash == i.drives[j].driveHash);
            require(Merkle.getRootWithDrive(_drives[j].position, _drives[j].log2Size, Merkle.getPristineHash(uint8(_drives[j].log2Size)), _drives[j].siblings) == initialHash, "Drive siblings must be compatible with previous initial hash for an empty drive");
            initialHash = Merkle.getRootWithDrive(_drives[j].position, _drives[j].log2Size, _drives[j].driveHash, _drives[j].siblings);
        }

        i.initialHash = initialHash;
        i.claimedFinalHash = _claimedFinalHash;
        i.currentState = state.WaitingConfirmation;

        emit ClaimSubmitted(_index, _claimedFinalHash);
    }

    /// @notice Is the given user concern about this instance.
    function isConcerned(uint256 _index, address _user) public view
        onlyInstantiated(_index)
        returns (bool)
    {
        DescartesCtx memory i = instance[_index];
        return (_user == i.claimer || _user == i.challenger);
    }

    /// @notice Get state of the instance concerning given user.
    function getState(uint256 _index, address _user) public view
        onlyInstantiated(_index)
        returns (
            bytes32,
            bytes32,
            uint256,
            uint256,
            uint256,
            bytes32,
            Drive[]
        )
    {
        DescartesCtx memory i = instance[_index];
        uint256 pendingDriveIndex = 0;
        if (i.state == state.WaitingProviders) {
            pendingDriveIndex = i.pendingDrives[i.pendingDrivesPointer];
        }
        return (
            i.initialHash,
            i.claimedFinalHash,
            pendingDriveIndex,
            i.finalTime,
            i.timeOfLastMove,
            getCurrentState(_index, _user),
            i.drives);
    }

    function getCurrentState(uint256 _index, address) public view
        onlyInstantiated(_index)
        returns (bytes32)
    {
        state memory currentState = instance[_index].currentState;
        if (currentState == state.WaitingProviders) {
            return "WaitingProviders";
        }
        if (currentState == state.ProviderMissedDeadline) {
            return "ProviderMissedDeadline";
        }
        if (currentState == state.WaitingClaim) {
            return "WaitingClaim";
        }
        if (currentState == state.WaitingClaim) {
            return "WaitingConfirmation";
        }
        if (currentState == state.WaitingChallenge) {
            return "WaitingChallenge";
        }
        if (currentState == state.ConsensusResult) {
            return "ConsensusResult";
        }

        require(false, "Unrecognized state");
    }

    /// @notice Get sub-instances of the instance
    function getSubInstances(uint256 _index, address) public view
        onlyInstantiated(_index)
        returns (address[] memory _addresses, uint256[] memory _indices)
    {
        address[] memory a;
        uint256[] memory i;

        if (instance[_index].currentState == state.WaitingChallenge) {
            a = new address[](1);
            i = new uint256[](1);
            a[0] = address(instance[_index].vg);
            i[0] = instance[_index].vgInstance;
        } else {
            a = new address[](0);
            i = new uint256[](0);
        }
        return (a, i);
    }

    /// @notice Claim the content of a integer drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to.
    /// @param _value integer value of the integer drive
    /// @param _siblings siblings of the integer drive of a pristine machine
    function claimIntegerDrive(uint256 _index, uint256 _value, bytes32[] _siblings) public onlyInstantiated(_index) {
        DecartesCtx storage i = instance[_index];
        require(i.currentState == state.WaitingProviders, "The state is not WaitingProviders");
        require(i.pendingDrivesPointer < i.pendingDrives.length, "No available pending drives");
        
        uint256 driveIndex = i.pendingDrives[pendingDrivesPointer];
        require(driveIndex < i.drives.length, "Invalid drive index");
        
        Drive storage drive = i.drives[driveIndex];
        require(drive.ready == false, "The drive shouldn't be ready");
        require(drive.provider == msg.sender, "The sender is not provider");
        require(drive.type == driveType.IntegerWithProvider, "The drive type is not IntegerWithProvider");
        require(Merkle.getRootWithDrive(drive.position, drive.log2Size, Merkle.getPristineHash(uint8(drive.log2Size)), siblings) == i.pristineHash, "Drive siblings must be compatible with pristine hash for an empty drive");

        bytes8 memory[] data = convertUint256ToBytes8Array(_value);
        bytes32 driveHash = li.calculateMerkleRootFromData(2, data);
        
        drive.ready = true;
        drive.driveHash = driveHash;
        drive.siblings = siblings;
        i.pendingDrivesPointer++;

        if (i.pendingDrivesPointer == i.pendingDrives.length) {
            i.currentState = state.WaitingClaim;
            i.timeOfLastMove = now;
        }
    }

    /// @notice Claim the hash of a logger drive (only drive provider can call it);
    ///         Or claim that the content is available on logger with given hash (any one can call it).
    /// @param _index index of Descartes instance the drive belongs to
    /// @param _value drive hash of the logger drive
    /// @param _siblings siblings of the logger drive of a pristine machine
    function claimLoggerDrive(uint256 _index, bytes32 _value, bytes32[] siblings) public onlyInstantiated(_index) {
        DecartesCtx storage i = instance[_index];
        require(i.currentState == state.WaitingProviders, "The state is not WaitingProviders");
        require(i.pendingDrivesPointer < i.pendingDrives.length, "No available pending drives");
        
        uint256 driveIndex = i.pendingDrives[pendingDrivesPointer];
        require(driveIndex < i.drives.length, "Invalid drive index");

        Drive storage drive = i.drives[driveIndex];
        require(drive.ready == false, "The drive shouldn't be ready");
        require((drive.type == driveType.LoggerWithProvider || drive.type == driveType.LoggerWithHash),
            "Drive is not logger type");

        if (drive.type == driveType.LoggerWithProvider) {
            require(drive.provider == msg.sender, "The sender is not provider");
            drive.bytesValue32 = _value;
            drive.type = driveType.LoggerWithHash;
        }

        if (drive.type == driveType.LoggerWithHash) {
            require(drive.bytesValue32 == _value, "Hash value doesn't match drive hash");

            if (li.isLogAvailable(drive.bytesValue32, drive.log2Size)) {
                require(Merkle.getRootWithDrive(drive.position, drive.log2Size, Merkle.getPristineHash(uint8(drive.log2Size)), siblings) == i.pristineHash, "Drive siblings must be compatible with pristine hash for an empty drive");
                
                drive.ready = true;
                drive.driveHash = drive.bytesValue32;
                drive.siblings = siblings;
                i.pendingDrivesPointer++;
            }
        }
        
        if (i.pendingDrivesPointer == i.pendingDrives.length) {
            i.currentState = state.WaitingClaim;
            i.timeOfLastMove = now;
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
        DescartesCtx memory i = instance[_index];
        require(i.currentState == state.WaitingChallenge, "State is not WaitingChallenge, cannot winByVG");
        uint256 vgIndex = i.vgInstance;

        if (i.vg.stateIsFinishedChallengerWon(vgIndex)) {
            challengerWins(_index);
            return;
        }

        if (i.vg.stateIsFinishedClaimerWon(vgIndex)) {
            claimerWins(_index);
            return;
        }
        require(false, "State of VG is not final");
    }
    
    /// @notice Abort the instance by missing deadline.
    /// @param _index index of Descartes instance to abort
    function abortByDeadline(uint256 _index) public onlyInstantiated(_index) {
        DescartesCtx storage i = instance[_index];
        bool afterDeadline = (now > i.timeOfLastMove + getMaxStateDuration(
                i.currentState,
                i.roundDuration,
                40, // time to start machine
                1, // vg is not instantiated, so it doesnt matter
                i.finalTime,
                500) // pico seconds to run instruction
            );

        require(afterDeadline, "Deadline is not over for this specific state");

        if (i.state == state.WaitingProviders) {
            i = state.ProviderMissedDeadline;
            return;
        }
        if (i.state == state.WaitingClaim) {
            i.currentState = state.ClaimerMisseddeadline;
            return;
        }
    }

    /// @notice Convert a uint256 to bytes8 array
    function convertUint256ToBytes8Array(uint256 _value) private returns(bytes8[]) {
        bytes8 memory[] data;
        for (uint256 i = 0; i < 4; i++) {
            bytes8 dataBytes8 = bytes8(_value << (j * 64) & 0xffffffffffffffff000000000000000000000000000000000000000000000000);
            data.push(dataBytes8);
        }
        return data;
    }
    
    /// @notice Get the worst case scenario duration for a specific state
    /// @param _roundDuration security parameter, the max time an agent
    //          has to react and submit one simple transaction
    /// @param _timeToStartMachine time to build the machine for the first time
    /// @param _partitionSize size of partition, how many instructions the
    //          will run to reach the necessary hash
    /// @param _maxCycle is the maximum amount of steps a machine can perform
    //          before being forced into becoming halted
    /// @param _picoSecondsToRunInsn time the offchain will take to run one instruction
    function getMaxStateDuration(
        state _state,
        VGInterface vg,
        uint256 _roundDuration,
        uint256 _timeToStartMachine,
        uint256 _partitionSize,
        uint256 _maxCycle,
        uint256 _picoSecondsToRunInsn
    ) private view returns (uint256)
    {
        if (_state == state.WaitingProviders) {
            // time to upload to logger + assemble pristine machine with drive
            uint256 maxLoggerUploadTime = 40 * 60;
            return _timeToStartMachine + maxLoggerUploadTime + _roundDuration;
        }

        if (_state == state.WaitingClaim) {
            // time to run entire machine + time to react
            return _timeToStartMachine + ((_maxCycle * _picoSecondsToRunInsn) / 1e12) + _roundDuration;
        }

        if (_state == state.WaitingConfirmation) {
            // time to run entire machine + time to react
            return _timeToStartMachine + ((_maxCycle * _picoSecondsToRunInsn) / 1e12) + _roundDuration;
        }

        if (_state == state.WaitingChallenge) {
            // time to run a verification game + time to react
            return vg.getMaxInstanceDuration(_roundDuration, _timeToStartMachine, _partitionSize, _maxCycle, _picoSecondsToRunInsn) + _roundDuration;
        }

        if (_state == state.ClaimerWon || _state == state.ChallengerWon || _state == state.ClaimerMissedDeadline || _state == state.ConsensusResult) {
            return 0; // final state
        }
    }
}

