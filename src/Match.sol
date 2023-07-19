// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./CanonicalConstants.sol";
import "./Tree.sol";
import "./Machine.sol";

/// @notice Implements functionalities to advance a match, until the point where divergence is found.
library Match {
    using Tree for Tree.Node;
    using Match for Id;
    using Match for IdHash;
    using Match for State;

    //
    // Id
    //

    struct Id {
        Tree.Node commitmentOne;
        Tree.Node commitmentTwo;
    }

    //
    // IdHash
    //
    type IdHash is bytes32;

    IdHash constant ZERO_ID = IdHash.wrap(bytes32(0x0));

    function hashFromId(Id memory id) internal pure returns (IdHash) {
        return IdHash.wrap(keccak256(abi.encode(id)));
    }

    function isZero(IdHash idHash) internal pure returns (bool) {
        return IdHash.unwrap(idHash) == 0x0;
    }

    function eq(IdHash left, IdHash right) internal pure returns (bool) {
        bytes32 l = IdHash.unwrap(left);
        bytes32 r = IdHash.unwrap(right);
        return l == r;
    }

    function requireEq(IdHash left, IdHash right) internal pure {
        require(left.eq(right), "matches are not equal");
    }

    function requireExist(IdHash idHash) internal pure {
        require(!idHash.isZero(), "match doesn't exist");
    }

    //
    // State
    //

    struct State {
        Tree.Node otherParent;
        Tree.Node leftNode;
        Tree.Node rightNode;
        // Once match is done, leftNode and rightNode change meaning
        // and contains contested final states.
        uint64 runningLeafPosition;
        uint64 height;
        uint64 currentHeight;
    }

    function createMatch(
        Tree.Node one,
        Tree.Node two,
        Tree.Node leftNodeOfTwo,
        Tree.Node rightNodeOfTwo,
        uint64 height
    ) internal pure returns (IdHash, State memory) {
        assert(two.verify(leftNodeOfTwo, rightNodeOfTwo));

        Id memory matchId = Id(one, two);

        State memory state = State(
            one,
            leftNodeOfTwo,
            rightNodeOfTwo,
            0,
            height, // TODO
            height // TODO
        );

        return (matchId.hashFromId(), state);
    }

    function goDownLeftTree(
        State storage state,
        Tree.Node newLeftNode,
        Tree.Node newRightNode
    ) internal {
        assert(state.currentHeight > 1);
        state.otherParent = state.leftNode;
        state.leftNode = newLeftNode;
        state.rightNode = newRightNode;

        state.currentHeight--;
    }

    function goDownRightTree(
        State storage state,
        Tree.Node newLeftNode,
        Tree.Node newRightNode
    ) internal {
        assert(state.currentHeight > 1);
        state.otherParent = state.rightNode;
        state.leftNode = newLeftNode;
        state.rightNode = newRightNode;

        state.runningLeafPosition += uint64(1 << state.currentHeight); // TODO: verify
        state.currentHeight--;
    }

    function setDivergenceOnLeftLeaf(
        State storage state,
        Tree.Node leftLeaf
    )
        internal
        returns (Machine.Hash finalStateOne, Machine.Hash finalStateTwo)
    {
        assert(state.currentHeight == 1);
        state.rightNode = leftLeaf;
        state.currentHeight = 0;

        if (state.height % 2 == 0) {
            finalStateOne = state.leftNode.toMachineHash();
            finalStateTwo = state.rightNode.toMachineHash();
        } else {
            finalStateOne = state.rightNode.toMachineHash();
            finalStateTwo = state.leftNode.toMachineHash();
        }
    }

    function setDivergenceOnRightLeaf(
        State storage state,
        Tree.Node rightLeaf
    )
        internal
        returns (Machine.Hash finalStateOne, Machine.Hash finalStateTwo)
    {
        assert(state.currentHeight == 1);
        state.leftNode = rightLeaf;
        state.runningLeafPosition += 1; // TODO: verify
        state.currentHeight = 0;

        if (state.height % 2 == 0) {
            finalStateOne = state.rightNode.toMachineHash();
            finalStateTwo = state.leftNode.toMachineHash();
        } else {
            finalStateOne = state.leftNode.toMachineHash();
            finalStateTwo = state.rightNode.toMachineHash();
        }
    }

    function getDivergence(
        State storage state
    )
        internal
        view
        returns (Machine.Hash finalStateOne, Machine.Hash finalStateTwo)
    {
        assert(state.currentHeight == 0);

        if (state.runningLeafPosition % 2 == 0) {
            // divergence was set on left leaf
            if (state.height % 2 == 0) {
                finalStateOne = state.leftNode.toMachineHash();
                finalStateTwo = state.rightNode.toMachineHash();
            } else {
                finalStateOne = state.rightNode.toMachineHash();
                finalStateTwo = state.leftNode.toMachineHash();
            }
        } else {
            // divergence was set on right leaf
            if (state.height % 2 == 0) {
                finalStateOne = state.rightNode.toMachineHash();
                finalStateTwo = state.leftNode.toMachineHash();
            } else {
                finalStateOne = state.leftNode.toMachineHash();
                finalStateTwo = state.rightNode.toMachineHash();
            }
        }
    }

    function setInitialState(
        State storage state,
        Machine.Hash initialState
    ) internal {
        state.otherParent = Tree.Node.wrap(Machine.Hash.unwrap(initialState));
    }

    //
    // View methods
    //

    function exists(State memory state) internal pure returns (bool) {
        return !state.otherParent.isZero();
    }

    function isFinished(State memory state) internal pure returns (bool) {
        return state.currentHeight == 0;
    }

    function canBeFinalized(State memory state) internal pure returns (bool) {
        return state.currentHeight == 1;
    }

    function canBeAdvanced(State memory state) internal pure returns (bool) {
        return state.currentHeight > 1;
    }

    function agreesOnLeftNode(
        State memory state,
        Tree.Node newLeftNode
    ) internal pure returns (bool) {
        return newLeftNode.eq(state.leftNode);
    }

    function toCycle(
        State memory state,
        uint256 startCycle,
        uint64 level
    ) internal pure returns (uint256) {
        uint64 log2step = ArbitrationConstants.log2step(level);
        return _toCycle(state, startCycle, log2step);
    }

    //
    // Requires
    //

    function requireExist(State memory state) internal pure {
        require(state.exists(), "match does not exist");
    }

    function requireIsFinished(State memory state) internal pure {
        require(state.isFinished(), "match is not finished");
    }

    function requireCanBeFinalized(State memory state) internal pure {
        require(state.canBeFinalized(), "match is not ready to be finalized");
    }

    function requireCanBeAdvanced(State memory state) internal pure {
        require(!state.canBeFinalized(), "match is ready to be finalized");
    }

    function requireParentHasChildren(
        State memory state,
        Tree.Node leftNode,
        Tree.Node rightNode
    ) internal pure {
        state.otherParent.requireChildren(leftNode, rightNode);
    }

    //
    // Private
    //

    function _toCycle(
        State memory state,
        uint256 base,
        uint64 log2step
    ) internal pure returns (uint256) {
        uint256 step = 1 << log2step;
        uint256 leafPosition = state.runningLeafPosition + 1; // +1 is implicit initialHash
        return base + (leafPosition * step); // TODO verify
    }
}
