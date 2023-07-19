// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Machine.sol";

library Tree {
    using Tree for Node;

    type Node is bytes32;

    Node constant ZERO_NODE = Node.wrap(bytes32(0x0));

    function eq(Node left, Node right) internal pure returns (bool) {
        bytes32 l = Node.unwrap(left);
        bytes32 r = Node.unwrap(right);
        return l == r;
    }

    function join(Node left, Node right) internal pure returns (Node) {
        bytes32 l = Node.unwrap(left);
        bytes32 r = Node.unwrap(right);
        bytes32 p = keccak256(abi.encode(l, r));
        return Node.wrap(p);
    }

    function verify(
        Node parent,
        Node left,
        Node right
    ) internal pure returns (bool) {
        return parent.eq(left.join(right));
    }

    function requireChildren(Node parent, Node left, Node right) internal pure {
        require(parent.verify(left, right), "child nodes don't match parent");
    }

    function isZero(Node node) internal pure returns (bool) {
        bytes32 n = Node.unwrap(node);
        return n == 0x0;
    }

    function requireExist(Node node) internal pure {
        require(!node.isZero(), "tree node doesn't exist");
    }

    function toMachineHash(Node node) internal pure returns (Machine.Hash) {
        return Machine.Hash.wrap(Node.unwrap(node));
    }
}
