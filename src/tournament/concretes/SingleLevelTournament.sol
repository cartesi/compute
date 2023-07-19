// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../abstracts/RootTournament.sol";
import "../abstracts/LeafTournament.sol";
import "step/contracts/interfaces/IUArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";

contract SingleLevelTournament is LeafTournament, RootTournament {
    constructor(
        Machine.Hash _initialHash,
        IUArchState _stateInterface,
        IUArchStep _stepInterface
    )
        LeafTournament(_stateInterface, _stepInterface)
        RootTournament(_initialHash)
    {}
}
