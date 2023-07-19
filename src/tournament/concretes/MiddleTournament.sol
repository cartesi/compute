// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../abstracts/NonLeafTournament.sol";
import "../abstracts/NonRootTournament.sol";

/// @notice Middle tournament is non-top, non-bottom of a multi-level instance
contract MiddleTournament is NonLeafTournament, NonRootTournament {
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
    )
        NonLeafTournament(IInnerTournamentFactory(msg.sender))
        NonRootTournament(
            _initialHash,
            _contestedCommitmentOne,
            _contestedFinalStateOne,
            _contestedCommitmentTwo,
            _contestedFinalStateTwo,
            _allowance,
            _startCycle,
            _level,
            _parent
        )
    {}
}
