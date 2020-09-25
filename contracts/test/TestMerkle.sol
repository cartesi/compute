// Copyright 2019 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

/// @title Library for Merkle proofs
pragma solidity ^0.7.0;

library Merkle {
    function getPristineHash(uint8 _log2Size)
        internal
        pure
        returns (bytes32 ret)
    {
        return ret;
    }

    function getRoot(
        uint64 _position,
        bytes8 _value,
        bytes32[] memory proof
    ) internal pure returns (bytes32 ret) {
        return ret;
    }

    function getRootWithDrive(
        uint64 _position,
        uint64 _logOfSize,
        bytes32 _drive,
        bytes32[] memory siblings
    ) internal pure returns (bytes32 ret) {
        return ret;
    }

    function getLog2Floor(uint256 number) internal pure returns (uint32 ret) {
        return ret;
    }

    function isPowerOf2(uint256 number) internal pure returns (bool) {
        return true;
    }

    /// @notice Calculate the root of Merkle tree from an array of power of 2 elements
    /// @param hashes The array containing power of 2 elements
    /// @return ret The root hash being calculated
    function calculateRootFromPowerOfTwo(bytes32[] memory hashes)
        internal
        pure
        returns (bytes32 ret)
    {
        return ret;
    }
}
