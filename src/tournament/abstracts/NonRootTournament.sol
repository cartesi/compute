// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Tournament.sol";
import "./NonLeafTournament.sol";

/// @notice Non-root tournament needs to propagate side-effects to its parent
abstract contract NonRootTournament is Tournament {
    using Machine for Machine.Hash;
    using Tree for Tree.Node;

    //
    // Constants
    //
    NonLeafTournament immutable parentTournament;

    Tree.Node immutable contestedCommitmentOne;
    Machine.Hash immutable contestedFinalStateOne;
    Tree.Node immutable contestedCommitmentTwo;
    Machine.Hash immutable contestedFinalStateTwo;

    //
    // Constructor
    //

    constructor(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level,
        NonLeafTournament _parent
    ) Tournament(_initialHash, _allowance, _startCycle, _level) {
        parentTournament = _parent;

        contestedCommitmentOne = _contestedCommitmentOne;
        contestedFinalStateOne = _contestedFinalStateOne;
        contestedCommitmentTwo = _contestedCommitmentTwo;
        contestedFinalStateTwo = _contestedFinalStateTwo;
    }

    /// @notice get the dangling commitment at current level and then retrieve the winner commitment
    function tournamentWinner() external view override returns (Tree.Node) {
        Tree.Node _danglingCommitment = _tournamentWinner();

        if (_danglingCommitment.isZero()) {
            return Tree.ZERO_NODE;
        }

        Machine.Hash _finalState = finalStates[_danglingCommitment];

        if (_finalState.eq(contestedFinalStateOne)) {
            return contestedCommitmentOne;
        } else {
            assert(_finalState.eq(contestedFinalStateTwo));
            return contestedCommitmentTwo;
        }
    }

    function updateParentTournamentDelay(
        Time.Instant _delay
    ) internal override {
        parentTournament.updateTournamentDelay(_delay);
    }

    /// @notice a final state is valid if it's equal to ContestedFinalStateOne or ContestedFinalStateTwo
    function validContestedFinalState(
        Machine.Hash _finalState
    ) internal view override returns (bool) {
        if (contestedFinalStateOne.eq(_finalState)) {
            return true;
        } else if (contestedFinalStateTwo.eq(_finalState)) {
            return true;
        } else {
            return false;
        }
    }
}
