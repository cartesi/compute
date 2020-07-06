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

/// @title DescartesInterface
/// @author Stephen Chen
pragma solidity ^0.5.0;
pragma experimental ABIEncoderV2;

import "@cartesi/util/contracts/Instantiator.sol";


contract DescartesInterface is Instantiator {

    enum State {
        WaitingProviders,
        ProviderMissedDeadline,
        WaitingReveals,
        ClaimerMissedDeadline,
        WaitingClaim,
        WaitingConfirmation,
        WaitingChallenge,
        ChallengerWon,
        ClaimerWon,
        ConsensusResult
    }

    /*
    There are two types of drive, one is directDrive, and the other is loggerDrive.
    directDrive has content inserted to the directValue field with up to 1MB;
    loggerDrive has content submitted to the logger contract,
    which can be retrieved with driveLog2Size and loggerRootHash.
    The needsLogger field is set to true for loggerDrive, false for directDrive.

    The waitsProvider field is set to true meaning the drive is not ready,
    and needs to be filled during the WaitingProviders phase.
    The provider field is the user who is responsible for filling out the drive.
    I.e the directValue of directDrive, or the loggerRootHash of loggerDrive
    */
    struct Drive {
        // start position of the drive
        uint64 position;
        // log2 size of the drive in the unit of bytes
        uint64 driveLog2Size;
        // direct value inserted to the drive
        bytes directValue;
        // root hash of the drive submitted to the logger
        bytes32 loggerRootHash;
        // the user who's responsible for filling out the drive
        address provider;
        // indicates the drive needs to wait for the provider to provide content
        bool waitsProvider;
        // indicates the content of the drive must be retrieved from logger
        bool needsLogger;
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
        uint256 _roundDuration,
        address _claimer,
        address _challenger,
        Drive[] memory _inputDrives) public returns (uint256);

    /// @notice Get result of a finished instance.
    /// @param _index index of Descartes instance to get result
    /// @return bool, indicates the result is ready
    /// @return bool, indicates the sdk is still running
    /// @return address, the user to blame for the abnormal stop of the sdk
    /// @return bytes32, the result of the sdk if available
    function getResult(uint256 _index) public view returns (
        bool,
        bool,
        address,
        bytes32);
}

