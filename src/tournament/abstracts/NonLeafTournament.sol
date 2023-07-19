// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../interfaces/IInnerTournamentFactory.sol";
import "./Tournament.sol";
import "./NonRootTournament.sol";

/// @notice Non-leaf tournament can create inner tournaments and matches
abstract contract NonLeafTournament is Tournament {
    using Clock for Clock.State;
    using Commitment for Tree.Node;
    using Machine for Machine.Hash;
    using Tree for Tree.Node;
    using Time for Time.Instant;
    using Match for Match.State;
    using Match for Match.Id;
    using Match for Match.IdHash;

    //
    // Constants
    //

    IInnerTournamentFactory immutable innerFactory;

    //
    // Storage
    //
    mapping(NonRootTournament => Match.IdHash) matchIdFromInnerTournaments;

    //
    // Events
    //

    event newInnerTournament(Match.IdHash indexed, NonRootTournament);

    //
    // Modifiers
    //

    modifier onlyInnerTournament() {
        Match.IdHash matchIdHash = matchIdFromInnerTournaments[
            NonRootTournament(msg.sender)
        ];
        matches[matchIdHash].requireExist();
        _;
    }

    //
    // Constructor
    //

    constructor(IInnerTournamentFactory _innerFactory) {
        innerFactory = _innerFactory;
    }

    function sealInnerMatchAndCreateInnerTournament(
        Match.Id calldata _matchId,
        Tree.Node _leftLeaf,
        Tree.Node _rightLeaf,
        Machine.Hash _initialHash,
        bytes32[] calldata _initialHashProof
    ) external tournamentNotFinished {
        Match.State storage _matchState = matches[_matchId.hashFromId()];
        _matchState.requireCanBeFinalized();
        _matchState.requireParentHasChildren(_leftLeaf, _rightLeaf);

        Machine.Hash _finalStateOne;
        Machine.Hash _finalStateTwo;

        if (!_matchState.agreesOnLeftNode(_leftLeaf)) {
            // Divergence is in the left leaf!
            (_finalStateOne, _finalStateTwo) = _matchState
                .setDivergenceOnLeftLeaf(_leftLeaf);
        } else {
            // Divergence is in the right leaf!
            (_finalStateOne, _finalStateTwo) = _matchState
                .setDivergenceOnRightLeaf(_rightLeaf);
        }

        // Pause clocks
        Time.Duration _maxDuration;
        {
            Clock.State storage _clock1 = clocks[_matchId.commitmentOne];
            Clock.State storage _clock2 = clocks[_matchId.commitmentTwo];
            _clock1.setPaused();
            _clock2.setPaused();
            _maxDuration = Clock.max(_clock1, _clock2);
        }

        // Prove initial hash is in commitment
        if (_matchState.runningLeafPosition == 0) {
            require(_initialHash.eq(initialHash), "initial hash incorrect");
        } else {
            _matchId.commitmentOne.proveHash(
                _matchState.runningLeafPosition - 1,
                _initialHash,
                _initialHashProof
            );
        }

        NonRootTournament _inner = innerFactory.instantiateInner(
            _initialHash,
            _matchId.commitmentOne,
            _finalStateOne,
            _matchId.commitmentTwo,
            _finalStateTwo,
            _maxDuration,
            _matchState.toCycle(startCycle, level),
            level + 1
        );
        matchIdFromInnerTournaments[_inner] = _matchId.hashFromId();

        emit newInnerTournament(_matchId.hashFromId(), _inner);
    }

    function winInnerMatch(
        NonRootTournament _childTournament,
        Tree.Node _leftNode,
        Tree.Node _rightNode
    ) external tournamentNotFinished {
        Match.IdHash _matchIdHash = matchIdFromInnerTournaments[_childTournament];
        _matchIdHash.requireExist();

        Match.State storage _matchState = matches[_matchIdHash];
        _matchState.requireExist();
        _matchState.requireIsFinished();

        Tree.Node _winner = _childTournament.tournamentWinner();
        _winner.requireExist();

        Tree.Node _commitmentRoot = _leftNode.join(_rightNode);
        require(_commitmentRoot.eq(_winner), "tournament winner is different");

        Clock.State storage _clock = clocks[_commitmentRoot];
        _clock.requireInitialized();
        _clock.addValidatorEffort(
            Time
                .currentTime()
                .timeSpan(_childTournament.maximumEnforceableDelay())
        );

        pairCommitment(
            _commitmentRoot,
            _clock,
            _leftNode,
            _rightNode
        );

        // delete storage
        delete matches[_matchIdHash];
        matchIdFromInnerTournaments[_childTournament] = Match.ZERO_ID;
    }


    function updateTournamentDelay(
        Time.Instant _delay
    ) external onlyInnerTournament {
        bool overrode = setMaximumDelay(_delay);
        if (overrode) {
            updateParentTournamentDelay(_delay);
        }
    }
}
