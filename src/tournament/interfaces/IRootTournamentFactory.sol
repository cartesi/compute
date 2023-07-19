// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.0 <0.9;

import "../interfaces/IInnerTournamentFactory.sol";
import "../abstracts/RootTournament.sol";

interface IRootTournamentFactory {
    event rootCreated(RootTournament);

    function instantiateSingle(
        Machine.Hash _initialHash
    ) external returns (RootTournament);

    function instantiateTopOfMultiple(
        Machine.Hash _initialHash
    ) external returns (RootTournament);
}
