// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.0 <0.9;

import "../abstracts/RootTournament.sol";
import "../abstracts/NonRootTournament.sol";

interface ISingleTournamentFactory {
    event rootCreated(RootTournament);

    function instantiateSingle(
        Machine.Hash _initialHash
    ) external returns (RootTournament);
}
