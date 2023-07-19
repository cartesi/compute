// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

library Machine {
    type Hash is bytes32;

    Hash constant ZERO_STATE = Hash.wrap(0x0);

    function notInitialized(Hash hash) internal pure returns (bool) {
        bytes32 h = Hash.unwrap(hash);
        return h == 0x0;
    }

    function eq(Hash left, Hash right) internal pure returns (bool) {
        bytes32 l = Hash.unwrap(left);
        bytes32 r = Hash.unwrap(right);
        return l == r;
    }

    type Cycle is uint256; // TODO overcomplicated?
    type Log2Step is uint64; // TODO overcomplicated?
}
