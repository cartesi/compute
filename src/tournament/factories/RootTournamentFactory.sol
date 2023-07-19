// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/IRootTournamentFactory.sol";
import "../concretes/SingleLevelTournament.sol";
import "../concretes/TopTournament.sol";
import "step/contracts/interfaces/IUArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";

contract RootTournamentFactory is IRootTournamentFactory {
    IInnerTournamentFactory immutable innerFactory;
    IUArchState immutable stateInterface;
    IUArchStep immutable stepInterface;

    constructor(
        IInnerTournamentFactory _innerFactory,
        IUArchState _stateInterface,
        IUArchStep _stepInterface
    ) {
        innerFactory = _innerFactory;
        stateInterface = _stateInterface;
        stepInterface = _stepInterface;
    }

    function instantiateSingle(
        Machine.Hash _initialHash
    ) external override returns (RootTournament) {
        SingleLevelTournament _tournament = new SingleLevelTournament(
            _initialHash,
            stateInterface,
            stepInterface
        );

        emit rootCreated(_tournament);

        return _tournament;
    }

    function instantiateTopOfMultiple(
        Machine.Hash _initialHash
    ) external override returns (RootTournament) {
        TopTournament _tournament = new TopTournament(
            innerFactory,
            _initialHash
        );

        emit rootCreated(_tournament);

        return _tournament;
    }
}
