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

contract TournamentTest is Test {
    using Tree for Tree.Node;
    using Time for Time.Instant;
    using Match for Match.Id;
    using Machine for Machine.Hash;

    // players' commitment node at different height
    // player 0, player 1, and player 2
    Tree.Node[][3] playerNodes;
    Tree.Node constant ONE_NODE = Tree.Node.wrap(bytes32(uint256(1)));

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
        playerNodes[1][0] = ONE_NODE;
        playerNodes[2][0] = ONE_NODE;

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

    function testJoinTournament() public {
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        // duplicate commitment should be reverted
        vm.expectRevert("clock is initialized");
        Util.joinTopTournament(playerNodes, topTournament, 0);

        // pair commitment, expect a match
        vm.expectEmit(true, true, false, true, address(topTournament));
        emit matchCreated(
            playerNodes[0][ArbitrationConstants.height(0) - 0],
            playerNodes[1][ArbitrationConstants.height(0) - 0],
            playerNodes[1][ArbitrationConstants.height(0) - 1]
        );
        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);
    }

    function testTimeout() public {
        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );

        uint256 _t = block.timestamp;
        // the delay is doubled when a match is created
        uint256 _tournamentFinishWithMatch = _t +
            1 +
            2 *
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);

        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        Match.Id memory _matchId = Util.matchId(playerNodes, 1, 0);
        assertFalse(
            topTournament.canWinMatchByTimeout(_matchId),
            "shouldn't be able to win match by timeout"
        );

        // player 1 should win after fast forward time to player 0 timeout
        // player 0 timeout first because he's supposed to advance match first after the match is created
        (Clock.State memory _player0Clock, ) = topTournament.getCommitment(
            playerNodes[0][ArbitrationConstants.height(0)]
        );
        vm.warp(
            Time.Instant.unwrap(
                _player0Clock.startInstant.add(_player0Clock.allowance)
            )
        );
        assertTrue(
            topTournament.canWinMatchByTimeout(_matchId),
            "should be able to win match by timeout"
        );
        topTournament.winMatchByTimeout(
            _matchId,
            playerNodes[1][ArbitrationConstants.height(0) - 1],
            playerNodes[1][ArbitrationConstants.height(0) - 1]
        );

        vm.warp(_tournamentFinishWithMatch);
        Tree.Node _winner = topTournament.tournamentWinner();
        (bool _finished, Machine.Hash _finalState) = topTournament
            .rootTournamentFinalState();

        assertTrue(
            _winner.eq(playerNodes[1][ArbitrationConstants.height(0)]),
            "winner should be player 1"
        );
        assertTrue(_finished, "tournament should be finished");
        assertTrue(_finalState.eq(Util.ONE_STATE), "final state should be 1");

        topTournament = Util.initializePlayer0Tournament(
            playerNodes,
            rootFactory
        );
        _t = block.timestamp;

        // the delay is doubled when a match is created
        _tournamentFinishWithMatch =
            _t +
            1 +
            2 *
            Time.Duration.unwrap(ArbitrationConstants.CENSORSHIP_TOLERANCE);

        // player 1 joins tournament
        Util.joinTopTournament(playerNodes, topTournament, 1);

        // player 0 should win after fast forward time to player 1 timeout
        // player 1 timeout first because he's supposed to advance match after player 0 advanced
        _matchId = Util.matchId(playerNodes, 1, 0);

        topTournament.advanceMatch(
            _matchId,
            playerNodes[0][ArbitrationConstants.height(0) - 1],
            playerNodes[0][ArbitrationConstants.height(0) - 1],
            playerNodes[0][ArbitrationConstants.height(0) - 2],
            playerNodes[0][ArbitrationConstants.height(0) - 2]
        );
        (Clock.State memory _player1Clock, ) = topTournament.getCommitment(
            playerNodes[1][ArbitrationConstants.height(0)]
        );
        vm.warp(
            Time.Instant.unwrap(
                _player1Clock.startInstant.add(_player1Clock.allowance)
            )
        );
        assertTrue(
            topTournament.canWinMatchByTimeout(_matchId),
            "should be able to win match by timeout"
        );
        topTournament.winMatchByTimeout(
            _matchId,
            playerNodes[0][ArbitrationConstants.height(0) - 1],
            playerNodes[0][ArbitrationConstants.height(0) - 1]
        );

        vm.warp(_tournamentFinishWithMatch);
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
    }
}
