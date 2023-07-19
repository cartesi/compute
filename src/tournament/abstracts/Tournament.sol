// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../../CanonicalConstants.sol";

import "../../Commitment.sol";
import "../../Time.sol";
import "../../Machine.sol";
import "../../Tree.sol";
import "../../Clock.sol";
import "../../Match.sol";

/// @notice Implements the core functionalities of a permissionless tournament that resolves
/// disputes of n parties in O(log(n))
/// @dev tournaments and matches are nested alternately. Anyone can join a tournament
/// while the tournament is still open, and two of the participants with unique commitments
/// will form a match. A match located in the last level is called `leafMatch`,
/// meaning the one-step disagreement is found and can be resolved by solidity-step.
/// Non-leaf (inner) matches would normally create inner tournaments with height = height + 1,
/// to find the divergence with improved precision.
abstract contract Tournament {
    using Machine for Machine.Hash;
    using Tree for Tree.Node;
    using Commitment for Tree.Node;

    using Time for Time.Instant;
    using Time for Time.Duration;

    using Clock for Clock.State;

    using Match for Match.Id;
    using Match for Match.IdHash;
    using Match for Match.State;

    //
    // Constants
    //

    Machine.Hash immutable initialHash;

    uint256 immutable startCycle;
    uint64 immutable level;

    Time.Instant immutable startInstant;
    Time.Duration immutable allowance;

    //
    // Storage
    //

    Time.Instant public maximumEnforceableDelay;
    Tree.Node danglingCommitment;

    mapping(Tree.Node => Clock.State) clocks;
    mapping(Tree.Node => Machine.Hash) finalStates;
    // matches existing in current tournament
    mapping(Match.IdHash => Match.State) matches;

    //
    // Events
    //

    event matchCreated(
        Tree.Node indexed one,
        Tree.Node indexed two,
        Tree.Node leftOfTwo
    );

    event matchAdvanced(Match.IdHash indexed, Tree.Node parent, Tree.Node left);

    //
    // Modifiers
    //

    modifier tournamentNotFinished() {
        require(!isFinished(), "tournament is finished");

        _;
    }

    modifier tournamentOpen() {
        require(!isClosed(), "tournament check-in elapsed");

        _;
    }

    //
    // Constructor
    //

    constructor(
        Machine.Hash _initialHash,
        Time.Duration _allowance,
        uint256 _startCycle,
        uint64 _level
    ) {
        initialHash = _initialHash;
        startCycle = _startCycle;
        level = _level;
        startInstant = Time.currentTime();
        allowance = _allowance;

        if (_allowance.gt(ArbitrationConstants.CENSORSHIP_TOLERANCE)) {
            maximumEnforceableDelay = Time.currentTime().add(
                ArbitrationConstants.CENSORSHIP_TOLERANCE
            );
        } else {
            maximumEnforceableDelay = Time.currentTime().add(_allowance);
        }
    }

    //
    // Methods
    //

    /// @dev root tournaments are open to everyone, while non-root tournaments are open to anyone who's final state hash matches the one of the two in the tournament
    function joinTournament(
        Machine.Hash _finalState,
        bytes32[] calldata _proof,
        Tree.Node _leftNode,
        Tree.Node _rightNode
    ) external tournamentOpen {
        Tree.Node _commitmentRoot = _leftNode.join(_rightNode);

        // Prove final state is in commitmentRoot
        _commitmentRoot.proveFinalState(level, _finalState, _proof);

        // Verify whether finalState is one of the two allowed of tournament if nested
        requireValidContestedFinalState(_finalState);
        finalStates[_commitmentRoot] = _finalState;

        Clock.State storage _clock = clocks[_commitmentRoot];
        _clock.requireNotInitialized(); // reverts if commitment is duplicate
        _clock.setNewPaused(startInstant, allowance);

        pairCommitment(_commitmentRoot, _clock, _leftNode, _rightNode);
    }

    /// @notice Advance the match until the smallest divergence is found at current level
    /// @dev this function is being called repeatedly in turns by the two parties that disagree on the commitment.
    function advanceMatch(
        Match.Id calldata _matchId,
        Tree.Node _leftNode,
        Tree.Node _rightNode,
        Tree.Node _newLeftNode,
        Tree.Node _newRightNode
    ) external tournamentNotFinished {
        Match.State storage _matchState = matches[_matchId.hashFromId()];
        _matchState.requireExist();
        _matchState.requireCanBeAdvanced();
        _matchState.requireParentHasChildren(_leftNode, _rightNode);

        if (!_matchState.agreesOnLeftNode(_leftNode)) {
            // go down left in Commitment tree
            _leftNode.requireChildren(_newLeftNode, _newRightNode);
            _matchState.goDownLeftTree(_newLeftNode, _newRightNode);
        } else {
            // go down right in Commitment tree
            _rightNode.requireChildren(_newLeftNode, _newRightNode);
            _matchState.goDownRightTree(_newLeftNode, _newRightNode);
        }

        // advance clocks
        clocks[_matchId.commitmentOne].advanceClock();
        clocks[_matchId.commitmentTwo].advanceClock();

        // TODO move event to lib?
        emit matchAdvanced(
            _matchId.hashFromId(),
            _matchState.otherParent,
            _matchState.leftNode
        );
    }

    function winMatchByTimeout(
        Match.Id calldata _matchId,
        Tree.Node _leftNode,
        Tree.Node _rightNode
    ) external tournamentNotFinished {
        matches[_matchId.hashFromId()].requireExist();
        Clock.State storage _clockOne = clocks[_matchId.commitmentOne];
        Clock.State storage _clockTwo = clocks[_matchId.commitmentTwo];

        _clockOne.requireInitialized();
        _clockTwo.requireInitialized();

        if (_clockOne.hasTimeLeft() && !_clockTwo.hasTimeLeft()) {
            require(
                _matchId.commitmentOne.verify(_leftNode, _rightNode),
                "child nodes do not match parent (commitmentOne)"
            );

            _clockOne.addValidatorEffort(_clockTwo.timeSinceTimeout());
            pairCommitment(
                _matchId.commitmentOne,
                _clockOne,
                _leftNode,
                _rightNode
            );
        } else if (!_clockOne.hasTimeLeft() && _clockTwo.hasTimeLeft()) {
            require(
                _matchId.commitmentTwo.verify(_leftNode, _rightNode),
                "child nodes do not match parent (commitmentTwo)"
            );

            _clockTwo.addValidatorEffort(_clockOne.timeSinceTimeout());
            pairCommitment(
                _matchId.commitmentTwo,
                _clockTwo,
                _leftNode,
                _rightNode
            );
        } else {
            revert("cannot win by timeout");
        }

        delete matches[_matchId.hashFromId()];
    }

    function _tournamentWinner() internal view returns (Tree.Node) {
        if (!isFinished()) {
            return Tree.ZERO_NODE;
        }

        (
            bool _hasDanglingCommitment,
            Tree.Node _danglingCommitment
        ) = hasDanglingCommitment();
        assert(_hasDanglingCommitment);

        return _danglingCommitment;
    }

    /// @return _winner commitment of the tournament
    function tournamentWinner() external view virtual returns (Tree.Node);

    //
    // View methods
    //

    function canWinMatchByTimeout(
        Match.Id calldata _matchId
    ) external view returns (bool) {
        Clock.State memory _clockOne = clocks[_matchId.commitmentOne];
        Clock.State memory _clockTwo = clocks[_matchId.commitmentTwo];

        return !_clockOne.hasTimeLeft() || !_clockTwo.hasTimeLeft();
    }

    function getCommitment(
        Tree.Node _commitmentRoot
    ) external view returns (Clock.State memory, Machine.Hash) {
        return (clocks[_commitmentRoot], finalStates[_commitmentRoot]);
    }

    function getMatch(
        Match.IdHash _matchIdHash
    ) public view returns (Match.State memory) {
        return matches[_matchIdHash];
    }

    function getMatchCycle(
        Match.IdHash _matchIdHash
    ) external view returns (uint256) {
        Match.State memory _m = getMatch(_matchIdHash);
        return _m.toCycle(startCycle, level);
    }

    function tournamentLevelConstants()
        external
        view
        returns (uint64 _level, uint64 _log2step, uint64 _height)
    {
        _level = level;
        _log2step = ArbitrationConstants.log2step(level);
        _height = ArbitrationConstants.height(level);
    }

    //
    // Helper functions
    //

    function hasDanglingCommitment()
        internal
        view
        returns (bool _h, Tree.Node _node)
    {
        _node = danglingCommitment;

        if (!_node.isZero()) {
            _h = true;
        }
    }

    function setDanglingCommitment(Tree.Node _node) internal {
        danglingCommitment = _node;
    }

    function clearDanglingCommitment() internal {
        danglingCommitment = Tree.ZERO_NODE;
    }

    function updateParentTournamentDelay(Time.Instant _delay) internal virtual;

    function setMaximumDelay(Time.Instant _delay) internal returns (bool) {
        if (_delay.gt(maximumEnforceableDelay)) {
            maximumEnforceableDelay = _delay;
            return true;
        } else {
            return false;
        }
    }

    function pairCommitment(
        Tree.Node _rootHash,
        Clock.State memory _newClock,
        Tree.Node _leftNode,
        Tree.Node _rightNode
    ) internal {
        (
            bool _hasDanglingCommitment,
            Tree.Node _danglingCommitment
        ) = hasDanglingCommitment();

        if (_hasDanglingCommitment) {
            (Match.IdHash _matchId, Match.State memory _matchState) = Match
                .createMatch(
                    _danglingCommitment,
                    _rootHash,
                    _leftNode,
                    _rightNode,
                    ArbitrationConstants.height(level)
                );

            matches[_matchId] = _matchState;

            Clock.State storage _firstClock = clocks[_danglingCommitment];
            Time.Instant _delay = Clock.deadline(_firstClock, _newClock);
            Time.Duration _maxDuration = Clock.max(_firstClock, _newClock);

            setMaximumDelay(_delay);
            updateParentTournamentDelay(_delay.add(_maxDuration)); // TODO hack

            _firstClock.advanceClock();

            clearDanglingCommitment();
            updateParentTournamentDelay(_delay);

            emit matchCreated(_danglingCommitment, _rootHash, _leftNode);
        } else {
            updateParentTournamentDelay(maximumEnforceableDelay.add(_newClock.allowance));
            setDanglingCommitment(_rootHash);
        }
    }

    /// @return bool if _fianlState is allowed to join the tournament
    function validContestedFinalState(
        Machine.Hash _fianlState
    ) internal view virtual returns (bool);

    function requireValidContestedFinalState(
        Machine.Hash _finalState
    ) internal view {
        require(
            validContestedFinalState(_finalState),
            "tournament doesn't have contested final state"
        );
    }

    /// @return bool if the tournament is still open to join
    function isClosed() internal view returns (bool) {
        if (allowance.gt(ArbitrationConstants.CENSORSHIP_TOLERANCE)) {
            return
                startInstant.timeoutElapsed(
                    ArbitrationConstants.CENSORSHIP_TOLERANCE
                );
        } else {
            return startInstant.timeoutElapsed(allowance);
        }
    }

    /// @return bool if the tournament is over
    function isFinished() internal view returns (bool) {
        return Time.currentTime().gt(maximumEnforceableDelay);
    }
}
