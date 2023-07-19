// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../abstracts/RootTournament.sol";
import "../abstracts/NonLeafTournament.sol";

/// @notice Top tournament of a multi-level instance
contract TopTournament is NonLeafTournament, RootTournament {
    constructor(
        IInnerTournamentFactory _innerFactory,
        Machine.Hash _initialHash
    )
        NonLeafTournament(_innerFactory)
        RootTournament(_initialHash)
    {}
}
