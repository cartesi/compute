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
pragma experimental ABIEncoderV2;

// #if BUILD_TEST
import './test/TestMerkle.sol';
// #else
import "@cartesi/util/contracts/Merkle.sol";
// #endif
import "@cartesi/util/contracts/Decorated.sol";
import "@cartesi/logger/contracts/LoggerInterface.sol";
import "@cartesi/arbitration/contracts/VGInterface.sol";
import "./DescartesInterface.sol";


contract Descartes is Decorated, DescartesInterface {
    address machine; // machine which will run the challenge
    LoggerInterface li;
    VGInterface vg;

    struct Challenge {
      address challenger; // First to challenge on this particular output
      bytes output; // do we want to just use the hasd of it?
    }

    struct ChallengeResolution {
      Challenge[] list;
      uint256 currentIdx;
      mapping(bytes => bool) alreadyChallenged;
      mapping(address => bool) hasVoted;
      uint votes;
    }

    struct DescartesCtx {
        address claimer; // selected claimer for claiming the machine output
        address[] challengers; // users who can challenge claimer's output
        ChallengeResolution challenges; // base for challenge resolution of multiple challengers
        Drive[] inputDrives;
        
        //@note below are the unchanged fields
        address owner; // the one who has power to shutdown the instance
        uint256 revealDrivesPointer; // the pointer to the current reveal drive
        uint256 providerDrivesPointer; // the pointer to the current provider drive
        uint256 finalTime; // max number of machine cycle to run
        uint64 outputPosition; // memory position of machine output
        uint64 outputLog2Size; // log2 size of the output drive in the unit of bytes
        uint256 roundDuration; // time interval to interact with this contract
        uint256 timeOfLastMove; // last time someone made a move with deadline
        uint256 vgInstance;
        bytes32 templateHash; // pristine hash of machine
        bytes32 initialHash; // initial hash with all drives mounted
        bytes32 claimedFinalHash; // claimed final hash of the machine
        bytes claimedOutput; // claimed final machine output
        State currentState;
        uint256[] revealDrives; // indices of the reveal drives
        uint256[] providerDrives; // indices of the provider drives
        bytes32[] driveHash; // root hash of the drives
    }

  

    mapping(uint256 => DescartesCtx) internal instance;
    /**@dev I haven't touched any of the state names yet , it's not really needed 
        but I thought those on the diagram bellow would make it more clear*/ 
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
    //   +---------Voting Phase-----------+      
    //   |   +-------------------------+  |     confirm    +-----------------+ 
    //   |   | AcceptingConfirmations  |--|--------------->| ConsensusResult | 
    //   |   +-------------------------+  |    or deadline +-----------------+ 
    //   |     |                          |
    //   |     |                          |
    //   |     | challenge                |
    //   |     v                          |
    //   |   +--------------------+       |   winByVG     +------------+ 
    //   |   | AcceptingChallenge |-------|-------------->| ClaimerWon | 
    //   |   +--------------------+       |               +------------+ 
    //   |     |                          |
    //   |     |                          |
    //   |     |                          |   winByVG        +---------------+                        |
    //   |     +--------------------------|----------------->| ChallengerWon |                        |
    //   |                                |                  +---------------+                        |
    //   +--------------------------------+

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
        address _machineAddress) public
    {
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
        uint64 _outputLog2Size,
        uint256 _roundDuration,
        address _claimer,
        address _challenger,
        Drive[] memory _inputDrives) public returns (uint256)


    /// @notice Challenger accepts claim.
    /// @param _index index of Descartes instance that the challenger is confirming the claim.
    function confirm(uint256 _index, bytes output) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].challenger)
        increasesNonce(_index) 
        // onlyVotingOpen(_index)
        {

            /**
                @dev very similar to challenge
                we need to check they are confirming with the *current* claimer, since it can fall out of sync
                (claimer has just lost by they indeed agreed with the losing side)

                if they don't agree with current claimer
                output != claimedOutput
                then it's actually a challenge, we then to avoid complications call challenge(_index, output)

                ELSE 
                    we just compute the "vote"
                    instance[_index].challenges.votes++;
                    AND also, if this is the last vote and NO CHALLENGES are going on, we say currentState = State.ClaimerWon

             */
        }


    /// @notice Challenger disputes the claim, starting a verification game.
    /// @param _index index of Descartes instance which challenger is starting the VG.
    function challenge(uint256 _index, bytes output) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].challenger)
        increasesNonce(_index) {
            /**
            @dev
            
            here we check if there are any challenges -- instance[_index].challenges.list.length == 0
            
            then if it was already added, we ignore -- instance[_index].alreadyChallenged[output]
            
            if not we add it -- instance[_index].challenges.list.push({msg.sender, output})
             
            in any case:  instance[_index].challenges.votes++;

             
            */

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
            /**
                @dev
                here we compute the claimers "vote" so we know when everyone has had their say
                instance[_index].challenges.votes++;

             */
        }


    /// @notice Is the given user concern about this instance.
    function isConcerned(uint256 _index, address _user) public view
        onlyInstantiated(_index)
        returns (bool)


    /// @notice Get state of the instance concerning given user.
    function getState(uint256 _index, address) public view
        onlyInstantiated(_index)
        returns (
            uint256[] memory,
            address[] memory,
            bytes32[] memory,
            bytes memory,
            Drive[] memory
        )


    function getCurrentState(uint256 _index) public view
        onlyInstantiated(_index)
        returns (bytes32)

    /// @notice Get sub-instances of the instance.
    function getSubInstances(uint256 _index, address) public view
        onlyInstantiated(_index)
        returns (address[] memory _addresses, uint256[] memory _indices)

    /// @notice Provide the content of a direct drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to.
    /// @param _value bytes value of the direct drive
    function provideDirectDrive(uint256 _index, bytes memory _value) public
        onlyInstantiated(_index)
        requirementsForProviderDrive(_index)

    /// @notice Provide the root hash of a logger drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to
    /// @param _root root hash of the logger drive
    function provideLoggerDrive(uint256 _index, bytes32 _root) public
        onlyInstantiated(_index)
        requirementsForProviderDrive(_index)

    /// @notice Reveal the content of a logger drive (only drive provider can call it).
    /// @param _index index of Descartes instance the drive belongs to
    function revealLoggerDrive(uint256 _index) public
        onlyInstantiated(_index)

    /// @notice In case one of the parties wins the verification game,
    ///         then he or she can call this function to claim victory in
    ///         this contract as well.
    /// @param _index index of Descartes instance to win
    function winByVG(uint256 _index) public
        onlyInstantiated(_index)
        increasesNonce(_index) {

            /**
            @dev here thera are not much changes, but instead of the being the invariable end leading the a final state of
            it can be a start to a new challenge.

            if instance[_index].challenges.currentIdx + 1 == instance[_index].challenges.list.length
            AND everybody "voted", THEN it's final and it behaves like it's right now

            if not then if Claimer won, we just increment instance[_index].challenges.currentIdx++
            @dev it means we are now open for a new challenge to be processed, or if none, just wait until all confirmations are over


            */
        }


    /// @notice Deactivate a Descartes SDK instance.
    /// @param _index index of Descartes instance to deactivate
    function destruct(uint256 _index) public
        onlyInstantiated(_index)
        onlyBy(instance[_index].owner)

    /// @notice Abort the instance by missing deadline.
    /// @param _index index of Descartes instance to abort
    function abortByDeadline(uint256 _index) public onlyInstantiated(_index) {
        /**
            @dev here we need to do the correct considerations regarding what it means to loose by deadline
            is it possible to miss deadline when running VG? Otherwise, no changes need to be made here
         */
    }

    /// @notice Get result of a finished instance.
    /// @param _index index of Descartes instance to get result
    /// @return bool, indicates the result is ready
    /// @return bool, indicates the sdk is still running
    /// @return address, the user to blame for the abnormal stop of the sdk
    /// @return bytes, the result of the sdk if available
    function getResult(uint256 _index) public view
        onlyInstantiated(_index)
        returns (bool, bool, address, bytes memory)


    /// @notice Convert bytes32 into bytes8[] and calculate the hashes of them
    function getWordHashesFromBytes32(bytes32 _value) private pure returns(bytes32[] memory) 

    /// @notice Convert bytes into bytes8[] and calculate the hashes of them
    function getWordHashesFromBytes(bytes memory _value) private pure returns(bytes32[] memory)

    /// @notice Get the worst case scenario duration for a specific state
    function getMaxStateDuration(
        uint256 _index
    ) private view returns (uint256)


    /// @notice several require statements for a drive
    modifier requirementsForProviderDrive(uint256 _index) 

    modifier onlyVotingOpen(_index) {
        require(instance[_index].challenges.votes < instance[_index].challenger.length + 1);
    }
}
