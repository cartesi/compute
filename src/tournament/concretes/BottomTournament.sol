// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../abstracts/LeafTournament.sol";
import "../abstracts/NonRootTournament.sol";
import "step/contracts/interfaces/IUArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";

/// @notice Bottom tournament of a multi-level instance
contract BottomTournament is LeafTournament, NonRootTournament {
    constructor(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level,
        IUArchState _stateInterface,
        IUArchStep _stepInterface,
        NonLeafTournament _parent
    )
        LeafTournament(_stateInterface, _stepInterface)
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
