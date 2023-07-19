// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/IInnerTournamentFactory.sol";
import "../concretes/MiddleTournament.sol";
import "../concretes/BottomTournament.sol";
import "step/contracts/interfaces/IUArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";

contract InnerTournamentFactory is IInnerTournamentFactory {
    IUArchState immutable stateInterface;
    IUArchStep immutable stepInterface;

    constructor(IUArchState _stateInterface, IUArchStep _stepInterface) {
        stateInterface = _stateInterface;
        stepInterface = _stepInterface;
    }

    function instantiateInner(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level
    ) external override returns (NonRootTournament) {
        // the inner tournament is bottom tournament at last level
        // else instantiate middle tournament
        NonRootTournament _tournament;
        if (_level == ArbitrationConstants.LEVELS - 1) {
            _tournament = instantiateBottom(
                _initialHash,
                _contestedCommitmentOne,
                _contestedFinalStateOne,
                _contestedCommitmentTwo,
                _contestedFinalStateTwo,
                _allowance,
                _startCycle,
                _level
            );
        } else {
            _tournament = instantiateMiddle(
                _initialHash,
                _contestedCommitmentOne,
                _contestedFinalStateOne,
                _contestedCommitmentTwo,
                _contestedFinalStateTwo,
                _allowance,
                _startCycle,
                _level
            );
        }

        return _tournament;
    }

    function instantiateMiddle(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level
    ) internal returns (NonRootTournament) {
        MiddleTournament _tournament = new MiddleTournament(
            _initialHash,
            _contestedCommitmentOne,
            _contestedFinalStateOne,
            _contestedCommitmentTwo,
            _contestedFinalStateTwo,
            _allowance,
            _startCycle,
            _level,
            NonLeafTournament(msg.sender)
        );

        return _tournament;
    }

    function instantiateBottom(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level
    ) internal returns (NonRootTournament) {
        BottomTournament _tournament = new BottomTournament(
            _initialHash,
            _contestedCommitmentOne,
            _contestedFinalStateOne,
            _contestedCommitmentTwo,
            _contestedFinalStateTwo,
            _allowance,
            _startCycle,
            _level,
            stateInterface,
            stepInterface,
            NonLeafTournament(msg.sender)
        );

        return _tournament;
    }
}
