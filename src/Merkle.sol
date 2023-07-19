// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

library Merkle {
    function getRootWithValue(
        uint64 _position,
        bytes8 _value,
        bytes32[] memory _proof
    ) public pure returns (bytes32) {
        bytes32 _runningHash = keccak256(abi.encodePacked(_value));

        return getRootWithDrive(_position, 3, _runningHash, _proof);
    }

    function getRootWithHash(
        uint64 _position,
        bytes32 _hash,
        bytes32[] memory _proof
    ) public pure returns (bytes32) {
        return getRootWithDrive(_position, 3, _hash, _proof);
    }

    function getRootWithDrive(
        uint64 _position,
        uint8 _logOfSize,
        bytes32 _drive,
        bytes32[] memory _siblings
    ) public pure returns (bytes32) {
        require(_logOfSize >= 3, "Must be at least a word");
        require(_logOfSize <= 64, "Cannot be bigger than the machine itself");

        uint64 _size = uint64(2) ** _logOfSize;

        require(((_size - 1) & _position) == 0, "Position is not aligned");
        require(
            _siblings.length == 64 - _logOfSize,
            "Proof length does not match"
        );

        for (uint64 _i = 0; _i < _siblings.length; _i++) {
            if ((_position & (_size << _i)) == 0) {
                _drive = keccak256(abi.encodePacked(_drive, _siblings[_i]));
            } else {
                _drive = keccak256(abi.encodePacked(_siblings[_i], _drive));
            }
        }

        return _drive;
    }
}
