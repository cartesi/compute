// Copyright 2023 Cartesi Pte. Ltd.

// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License"); you may not use
// this file except in compliance with the License. You may obtain a copy of the
// License at http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software distributed
// under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
// CONDITIONS OF ANY KIND, either express or implied. See the License for the
// specific language governing permissions and limitations under the License.

import "src/Match.sol";
import "src/CanonicalConstants.sol";
import "src/tournament/concretes/TopTournament.sol";
import "src/tournament/concretes/MiddleTournament.sol";
import "src/tournament/interfaces/IRootTournamentFactory.sol";

pragma solidity ^0.8.0;

library Util {
    using Tree for Tree.Node;
    using Machine for Machine.Hash;
    using Match for Match.Id;
    using Match for Match.State;

    Tree.Node constant ONE_NODE = Tree.Node.wrap(bytes32(uint256(1)));
    Machine.Hash constant ONE_STATE = Machine.Hash.wrap(bytes32(uint256(1)));
    Machine.Hash constant TWO_STATE = Machine.Hash.wrap(bytes32(uint256(2)));

    function generateProof(
        Tree.Node[][3] memory _playerNodes,
        uint256 _player,
        uint64 _height
    ) internal pure returns (bytes32[] memory) {
        bytes32[] memory _proof = new bytes32[](_height);
        for (uint64 _i = 0; _i < _height; _i++) {
            _proof[_i] = Tree.Node.unwrap(_playerNodes[_player][_i]);
        }
        return _proof;
    }

    // advance match between player 0 and player 1
    function advanceMatch01AtLevel(
        Tree.Node[][3] memory _playerNodes,
        TopTournament _topTournament,
        Match.Id memory _matchId,
        uint64 _level
    ) internal returns (uint256 _playerToSeal) {
        uint256 _current = ArbitrationConstants.height(_level);
        for (_current; _current > 1; _current -= 1) {
            if (_playerToSeal == 0) {
                // advance match alternately until it can be sealed
                // starts with player 0
                _topTournament.advanceMatch(
                    _matchId,
                    _playerNodes[0][_current - 1],
                    _playerNodes[0][_current - 1],
                    _playerNodes[0][_current - 2],
                    _playerNodes[0][_current - 2]
                );
                _playerToSeal = 1;
            } else {
                _topTournament.advanceMatch(
                    _matchId,
                    _playerNodes[1][_current - 1],
                    _playerNodes[1][_current - 1],
                    _playerNodes[1][_current - 2],
                    _playerNodes[1][_current - 2]
                );
                _playerToSeal = 0;
            }
        }
    }

    // advance match between player 0 and player 2
    function advanceMatch02AtLevel(
        Tree.Node[][3] memory _playerNodes,
        TopTournament _topTournament,
        Match.Id memory _matchId,
        uint64 _level
    ) internal returns (uint256 _playerToSeal) {
        uint256 _current = ArbitrationConstants.height(_level);
        for (_current; _current > 1; _current -= 1) {
            if (_playerToSeal == 0) {
                // advance match alternately until it can be sealed
                // starts with player 0
                _topTournament.advanceMatch(
                    _matchId,
                    _playerNodes[0][_current - 1],
                    _playerNodes[0][_current - 1],
                    _playerNodes[0][_current - 2],
                    _playerNodes[0][_current - 2]
                );
                _playerToSeal = 2;
            } else {
                _topTournament.advanceMatch(
                    _matchId,
                    _playerNodes[0][_current - 1],
                    _playerNodes[2][_current - 1],
                    _playerNodes[0][_current - 2],
                    _playerNodes[2][_current - 2]
                );
                _playerToSeal = 0;
            }
        }
    }

    // create new _topTournament and player 0 joins it
    function initializePlayer0Tournament(
        Tree.Node[][3] memory _playerNodes,
        IRootTournamentFactory _rootFactory
    ) internal returns (TopTournament _topTournament) {
        _topTournament = TopTournament(
            address(_rootFactory.instantiateTopOfMultiple(Machine.ZERO_STATE))
        );
        // player 0 joins tournament
        joinTopTournament(_playerNodes, _topTournament, 0);
    }

    // _player joins _topTournament
    function joinTopTournament(
        Tree.Node[][3] memory _playerNodes,
        TopTournament _topTournament,
        uint256 _player
    ) internal {
        if (_player == 0) {
            _topTournament.joinTournament(
                Machine.ZERO_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(0)
                ),
                _playerNodes[0][ArbitrationConstants.height(0) - 1],
                _playerNodes[0][ArbitrationConstants.height(0) - 1]
            );
        } else if (_player == 1) {
            _topTournament.joinTournament(
                ONE_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(0)
                ),
                _playerNodes[1][ArbitrationConstants.height(0) - 1],
                _playerNodes[1][ArbitrationConstants.height(0) - 1]
            );
        } else if (_player == 2) {
            _topTournament.joinTournament(
                TWO_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(0)
                ),
                _playerNodes[0][ArbitrationConstants.height(0) - 1],
                _playerNodes[2][ArbitrationConstants.height(0) - 1]
            );
        }
    }

    // _player joins _middleTournament at _level
    function joinMiddleTournament(
        Tree.Node[][3] memory _playerNodes,
        MiddleTournament _middleTournament,
        uint256 _player,
        uint64 _level
    ) internal {
        if (_player == 0) {
            _middleTournament.joinTournament(
                Machine.ZERO_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(_level)
                ),
                _playerNodes[0][ArbitrationConstants.height(_level) - 1],
                _playerNodes[0][ArbitrationConstants.height(_level) - 1]
            );
        } else if (_player == 1) {
            _middleTournament.joinTournament(
                ONE_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(_level)
                ),
                _playerNodes[1][ArbitrationConstants.height(_level) - 1],
                _playerNodes[1][ArbitrationConstants.height(_level) - 1]
            );
        } else if (_player == 2) {
            _middleTournament.joinTournament(
                TWO_STATE,
                generateProof(
                    _playerNodes,
                    _player,
                    ArbitrationConstants.height(_level)
                ),
                _playerNodes[0][ArbitrationConstants.height(_level) - 1],
                _playerNodes[2][ArbitrationConstants.height(_level) - 1]
            );
        }
    }

    // create match id for player 0 and _opponent at _level
    function matchId(
        Tree.Node[][3] memory _playerNodes,
        uint256 _opponent,
        uint64 _level
    ) internal pure returns (Match.Id memory) {
        return
            Match.Id(
                _playerNodes[0][ArbitrationConstants.height(_level)],
                _playerNodes[_opponent][ArbitrationConstants.height(_level)]
            );
    }
}
