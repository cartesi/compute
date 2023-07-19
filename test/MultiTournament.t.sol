// Copyright 2023 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "./Util.sol";
import "src/tournament/factories/RootTournamentFactory.sol";
import "src/tournament/factories/InnerTournamentFactory.sol";
import "src/CanonicalConstants.sol";
import "step/contracts/UArchStep.sol";
import "step/contracts/UArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";
import "step/contracts/interfaces/IUArchState.sol";

pragma solidity ^0.8.0;

contract MultiTournamentTest is Test {
    using Tree for Tree.Node;
    using Time for Time.Instant;
    using Match for Match.Id;
    using Machine for Machine.Hash;

    // players' commitment node at different height
    // player 0, player 1, and player 2
    Tree.Node[][3] playerNodes;

    IUArchState immutable state;
    IUArchStep immutable step;
    IRootTournamentFactory immutable rootFactory;
    IInnerTournamentFactory immutable innerFactory;
    TopTournament topTournament;
    MiddleTournament middleTournament;

    event matchCreated(
        Tree.Node indexed one,
        Tree.Node indexed two,
        Tree.Node leftOfTwo
    );
    event newInnerTournament(Match.IdHash indexed, NonRootTournament);

    constructor() {
        state = new UArchState();
        step = new UArchStep();
        innerFactory = new InnerTournamentFactory(state, step);
        rootFactory = new RootTournamentFactory(innerFactory, state, step);
    }

    function setUp() public {
        playerNodes[0] = new Tree.Node[](ArbitrationConstants.height(0) + 1);
        playerNodes[1] = new Tree.Node[](ArbitrationConstants.height(0) + 1);
        playerNodes[2] = new Tree.Node[](ArbitrationConstants.height(0) + 1);

        playerNodes[0][0] = Tree.ZERO_NODE;
        playerNodes[1][0] = Util.ONE_NODE;
        playerNodes[2][0] = Util.ONE_NODE;

        for (uint256 _i = 1; _i <= ArbitrationConstants.height(0); _i++) {
            // player 0 is all zero leaf node
            playerNodes[0][_i] = playerNodes[0][_i - 1].join(
                playerNodes[0][_i - 1]
            );
            // player 1 is all 1
            playerNodes[1][_i] = playerNodes[1][_i - 1].join(
                playerNodes[1][_i - 1]
            );
            // player 2 is all 0 but right most leaf node is 1
            playerNodes[2][_i] = playerNodes[0][_i - 1].join(
                playerNodes[2][_i - 1]
            );
        }
    }

    function testRootWinner() public {
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // no winner before tournament finished
        Tree.Node _winner = topTournament.tournamentWinner();
        (bool _finished, Machine.Hash _finalState) = topTournament
            .rootTournamentFinalState();

        assertTrue(_winner.isZero(), "winner should be zero node");
        assertFalse(_finished, "tournament shouldn't be finished");
        assertTrue(
            _finalState.eq(Machine.ZERO_STATE),
            "final state should be zero"
        );

        // player 0 should win after fast forward time to tournament finishes
        uint256 _t = block.timestamp;
        uint256 _tournamentFinish = _t +
            1 +
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);

        // the delay is doubled when a match is created
        uint256 _tournamentFinishWithMatch = _tournamentFinish +
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);

        vm.warp(_tournamentFinish);
        _winner = topTournament.tournamentWinner();
        (_finished, _finalState) = topTournament.rootTournamentFinalState();

        assertTrue(
            _winner.eq(playerNodes[0][ArbitrationConstants.height(0)]),
            "winner should be player 0"
        );
        assertTrue(_finished, "tournament should be finished");
        assertTrue(
            _finalState.eq(Tree.ZERO_NODE.toMachineHash()),
            "final state should be zero"
        );

        // rewind time in half and pair commitment, expect a match
        vm.warp(_t);
        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        // no dangling commitment available, should revert
        vm.warp(_tournamentFinishWithMatch);
        vm.expectRevert();
        _winner = topTournament.tournamentWinner();
    }

    function testInner() public {
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // pair commitment, expect a match
        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        Match.Id memory _matchId = Util.matchId(playerNodes, 1, 0);

        // advance match to end, this match will always advance to left tree
        uint256 _playerToSeal = Util.advanceMatch01AtLevel(
            playerNodes,
            topTournament,
            _matchId,
            0
        );

        // seal match
        topTournament.sealInnerMatchAndCreateInnerTournament(
            _matchId,
            playerNodes[_playerToSeal][0],
            playerNodes[_playerToSeal][0],
            Machine.ZERO_STATE,
            Util.generateProof(
                playerNodes,
                _playerToSeal,
                ArbitrationConstants.height(1)
            )
        );

        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // pair commitment, expect a match
        // player 2 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 2);

        _matchId = Util.matchId(playerNodes, 2, 0);

        // advance match to end, this match will always advance to right tree
        _playerToSeal = Util.advanceMatch02AtLevel(
            playerNodes,
            topTournament,
            _matchId,
            0
        );

        // seal match
        topTournament.sealInnerMatchAndCreateInnerTournament(
            _matchId,
            playerNodes[0][0],
            playerNodes[_playerToSeal][0],
            Machine.ZERO_STATE,
            Util.generateProof(
                playerNodes,
                _playerToSeal,
                ArbitrationConstants.height(1)
            )
        );
    }

    function testInnerWinner() public {
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // pair commitment, expect a match
        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        Match.Id memory _matchId = Util.matchId(playerNodes, 1, 0);

        // advance match to end, this match will always advance to left tree
        uint256 _playerToSeal = Util.advanceMatch01AtLevel(
            playerNodes,
            topTournament,
            _matchId,
            0
        );

        // expect new inner created
        vm.recordLogs();

        // seal match
        topTournament.sealInnerMatchAndCreateInnerTournament(
            _matchId,
            playerNodes[_playerToSeal][0],
            playerNodes[_playerToSeal][0],
            Machine.ZERO_STATE,
            Util.generateProof(
                playerNodes,
                _playerToSeal,
                ArbitrationConstants.height(1)
            )
        );

        Vm.Log[] memory _entries = vm.getRecordedLogs();
        assertEq(_entries[0].topics.length, 2);
        assertEq(
            _entries[0].topics[0],
            keccak256("newInnerTournament(bytes32,address)")
        );
        assertEq(
            _entries[0].topics[1],
            Match.IdHash.unwrap(_matchId.hashFromId())
        );

        middleTournament = MiddleTournament(
            address(bytes20(bytes32(_entries[0].data) << (12 * 8)))
        );

        Tree.Node _winner = middleTournament.tournamentWinner();
        assertTrue(_winner.isZero(), "winner should be zero node");

        // player 0 should win after fast forward time to inner tournament finishes
        uint256 _t = block.timestamp;
        // the delay is doubled when a match is created
        uint256 _rootTournamentFinish = _t +
            2 *
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);
        Util.joinMiddleTournament(playerNodes, middleTournament, 0, 1);

        vm.warp(_rootTournamentFinish - 1);
        _winner = middleTournament.tournamentWinner();
        topTournament.winInnerMatch(
            middleTournament,
            playerNodes[0][ArbitrationConstants.height(0) - 1],
            playerNodes[0][ArbitrationConstants.height(0) - 1]
        );

        vm.warp(_rootTournamentFinish + 1);
        (bool _finished, Machine.Hash _finalState) = topTournament
            .rootTournamentFinalState();

        assertTrue(
            _winner.eq(playerNodes[0][ArbitrationConstants.height(0)]),
            "winner should be player 0"
        );
        assertTrue(_finished, "tournament should be finished");
        assertTrue(
            _finalState.eq(Tree.ZERO_NODE.toMachineHash()),
            "final state should be zero"
        );

        //create another tournament for other test
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // pair commitment, expect a match
        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        _matchId = Util.matchId(playerNodes, 1, 0);

        // advance match to end, this match will always advance to left tree
        _playerToSeal = Util.advanceMatch01AtLevel(
            playerNodes,
            topTournament,
            _matchId,
            0
        );

        // expect new inner created
        vm.recordLogs();

        // seal match
        topTournament.sealInnerMatchAndCreateInnerTournament(
            _matchId,
            playerNodes[_playerToSeal][0],
            playerNodes[_playerToSeal][0],
            Machine.ZERO_STATE,
            Util.generateProof(
                playerNodes,
                _playerToSeal,
                ArbitrationConstants.height(1)
            )
        );

        _entries = vm.getRecordedLogs();
        assertEq(_entries[0].topics.length, 2);
        assertEq(
            _entries[0].topics[0],
            keccak256("newInnerTournament(bytes32,address)")
        );
        assertEq(
            _entries[0].topics[1],
            Match.IdHash.unwrap(_matchId.hashFromId())
        );

        middleTournament = MiddleTournament(
            address(bytes20(bytes32(_entries[0].data) << (12 * 8)))
        );

        _winner = middleTournament.tournamentWinner();
        assertTrue(_winner.isZero(), "winner should be zero node");

        _t = block.timestamp;
        // the delay is doubled when a match is created
        uint256 _middleTournamentFinish = _t +
            1 +
            2 *
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);
        _rootTournamentFinish =
            _middleTournamentFinish +
            2 *
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);

        Util.joinMiddleTournament(playerNodes, middleTournament, 0, 1);

        //let player 1 join, then timeout player 0
        Util.joinMiddleTournament(playerNodes, middleTournament, 1, 1);

        (Clock.State memory _player0Clock, ) = middleTournament.getCommitment(
            playerNodes[0][ArbitrationConstants.height(1)]
        );
        vm.warp(
            Time.Instant.unwrap(
                _player0Clock.startInstant.add(_player0Clock.allowance)
            )
        );
        _matchId = Util.matchId(playerNodes, 1, 1);
        middleTournament.winMatchByTimeout(
            _matchId,
            playerNodes[1][ArbitrationConstants.height(1) - 1],
            playerNodes[1][ArbitrationConstants.height(1) - 1]
        );

        vm.warp(_middleTournamentFinish);
        _winner = middleTournament.tournamentWinner();
        topTournament.winInnerMatch(
            middleTournament,
            playerNodes[1][ArbitrationConstants.height(0) - 1],
            playerNodes[1][ArbitrationConstants.height(0) - 1]
        );

        vm.warp(_rootTournamentFinish);
        (_finished, _finalState) = topTournament.rootTournamentFinalState();

        assertTrue(
            _winner.eq(playerNodes[1][ArbitrationConstants.height(0)]),
            "winner should be player 1"
        );
        assertTrue(_finished, "tournament should be finished");
        assertTrue(
            _finalState.eq(Util.ONE_NODE.toMachineHash()),
            "final state should be 1"
        );
    }
}
