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

import "src/tournament/abstracts/RootTournament.sol";
import "src/tournament/factories/RootTournamentFactory.sol";
import "src/tournament/factories/InnerTournamentFactory.sol";
import "src/CanonicalConstants.sol";
import "step/contracts/UArchStep.sol";
import "step/contracts/UArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";
import "step/contracts/interfaces/IUArchState.sol";

pragma solidity ^0.8.0;

contract TournamentFactoryTest is Test {
    IRootTournamentFactory rootFactory;
    IInnerTournamentFactory innerFactory;
    IUArchState state = new UArchState();
    IUArchStep step = new UArchStep();

    function setUp() public {
        innerFactory = new InnerTournamentFactory(state, step);
        rootFactory = new RootTournamentFactory(innerFactory, state, step);
    }

    function testRootTournament() public {
        RootTournament rootTournament = rootFactory.instantiateSingle(
            Machine.ZERO_STATE
        );

        (uint64 _level, uint64 _log2step, uint64 _height) = rootTournament
            .tournamentLevelConstants();

        assertEq(_level, 0, "level should be 0");
        assertEq(
            _log2step,
            ArbitrationConstants.log2step(_level),
            "log2step should match"
        );
        assertEq(
            _height,
            ArbitrationConstants.height(_level),
            "height should match"
        );

        rootTournament = rootFactory.instantiateTopOfMultiple(
            Machine.ZERO_STATE
        );

        (_level, _log2step, _height) = rootTournament
            .tournamentLevelConstants();

        assertEq(_level, 0, "level should be 0");
        assertEq(
            _log2step,
            ArbitrationConstants.log2step(_level),
            "log2step should match"
        );
        assertEq(
            _height,
            ArbitrationConstants.height(_level),
            "height should match"
        );
    }
}
