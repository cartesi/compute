// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.0 <0.9;

import "../abstracts/RootTournament.sol";
import "../abstracts/NonRootTournament.sol";

interface IMultiTournamentFactory {
    event rootCreated(RootTournament);

    function instantiateTop(
        Machine.Hash _initialHash
    ) external returns (RootTournament);

    function instantiateInner(
        Machine.Hash _initialHash,
        Tree.Node _contestedCommitmentOne,
        Machine.Hash _contestedFinalStateOne,
        Tree.Node _contestedCommitmentTwo,
        Machine.Hash _contestedFinalStateTwo,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level
    ) external returns (NonRootTournament);
}
