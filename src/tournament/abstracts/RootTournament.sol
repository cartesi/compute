// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Tournament.sol";

/// @notice Root tournament has no parent
abstract contract RootTournament is Tournament {
    //
    // Constructor
    //

    constructor(
        Machine.Hash _initialHash
    ) Tournament(_initialHash, ArbitrationConstants.CENSORSHIP_TOLERANCE, 0, 0) {}

    function tournamentWinner() external view override returns (Tree.Node) {
        return _tournamentWinner();
    }

    function updateParentTournamentDelay(
        Time.Instant _delay
    ) internal override {
        // do nothing, the root tournament has no parent to update
    }

    function validContestedFinalState(
        Machine.Hash
    ) internal pure override returns (bool) {
        // always returns true in root tournament
        return true;
    }

    function rootTournamentFinalState() external view returns (bool, Machine.Hash) {
        if (!isFinished()) {
            return (false, Machine.ZERO_STATE);
        }

        (bool _hasDanglingCommitment, Tree.Node _danglingCommitment) =
            hasDanglingCommitment();
        assert(_hasDanglingCommitment);

        Machine.Hash _finalState = finalStates[_danglingCommitment];
        return (true, _finalState);
    }
}
