// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Tournament.sol";
import "../../Commitment.sol";
import "../../Merkle.sol";
import "step/contracts/interfaces/IUArchState.sol";
import "step/contracts/interfaces/IUArchStep.sol";
import "step/contracts/interfaces/IMemoryAccessLog.sol";

/// @notice Leaf tournament is the one that seals leaf match
abstract contract LeafTournament is Tournament {
    using Machine for Machine.Hash;
    using Commitment for Tree.Node;
    using Tree for Tree.Node;
    using Clock for Clock.State;
    using Match for Match.Id;
    using Match for Match.State;

    IUArchState immutable stateInterface;
    IUArchStep immutable stepInterface;

    constructor(IUArchState _stateInterface, IUArchStep _stepInterface) {
        stateInterface = _stateInterface;
        stepInterface = _stepInterface;
    }

    function sealLeafMatch(
        Match.Id calldata _matchId,
        Tree.Node _leftLeaf,
        Tree.Node _rightLeaf,
        Machine.Hash _initialHash,
        bytes32[] calldata _initialHashProof
    ) external tournamentNotFinished {
        Match.State storage _matchState = matches[_matchId.hashFromId()];
        _matchState.requireExist();
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

        // Unpause clocks
        Clock.State storage _clock1 = clocks[_matchId.commitmentOne];
        Clock.State storage _clock2 = clocks[_matchId.commitmentTwo];
        _clock1.setPaused();
        _clock1.advanceClock();
        _clock2.setPaused();
        _clock2.advanceClock();

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

        _matchState.setInitialState(_initialHash);
    }

    // TODO: do validate access logs, do solidity-step
    function winLeafMatch(
        Match.Id calldata _matchId,
        IMemoryAccessLog.AccessLogs calldata _accessLogs,
        bytes32[] calldata _oldHashes,
        bytes32[][] calldata _proofs,
        Tree.Node _leftNode,
        Tree.Node _rightNode
    ) external tournamentNotFinished {
        Match.State storage _matchState = matches[_matchId.hashFromId()];
        _matchState.requireExist();
        _matchState.requireIsFinished();

        Clock.State storage _clockOne = clocks[_matchId.commitmentOne];
        Clock.State storage _clockTwo = clocks[_matchId.commitmentTwo];
        _clockOne.requireInitialized();
        _clockTwo.requireInitialized();

        require(
            _accessLogs.logs.length == _proofs.length,
            "proofs length doesn't match"
        );

        {
            // workaround stack too deep problem
            Machine.Hash _finalState = executeStep(
                _matchState.otherParent.toMachineHash(),
                _accessLogs,
                _oldHashes,
                _proofs
            );

            (
                Machine.Hash _finalStateOne,
                Machine.Hash _finalStateTwo
            ) = _matchState.getDivergence();

            if (_leftNode.join(_rightNode).eq(_matchId.commitmentOne)) {
                require(
                    _finalState.eq(_finalStateOne),
                    "final state one doesn't match"
                );

                _clockOne.addValidatorEffort(Time.ZERO_DURATION);
                pairCommitment(
                    _matchId.commitmentOne,
                    _clockOne,
                    _leftNode,
                    _rightNode
                );
            } else if (_leftNode.join(_rightNode).eq(_matchId.commitmentTwo)) {
                require(
                    _finalState.eq(_finalStateTwo),
                    "final state two doesn't match"
                );

                _clockTwo.addValidatorEffort(Time.ZERO_DURATION);
                pairCommitment(
                    _matchId.commitmentTwo,
                    _clockTwo,
                    _leftNode,
                    _rightNode
                );
            } else {
                revert("wrong left/right nodes for step");
            }
        }

        delete matches[_matchId.hashFromId()];
    }

    function executeStep(
        Machine.Hash _initialState,
        IMemoryAccessLog.AccessLogs memory _accessLogs,
        bytes32[] memory _oldHashes,
        bytes32[][] memory _proofs
    ) internal returns (Machine.Hash) {
        uint256 _writeCount = 0;
        Machine.Hash _finalState = _initialState;
        for (uint256 _i = 0; _i < _accessLogs.logs.length; _i++) {
            if (
                _accessLogs.logs[_i].accessType ==
                IMemoryAccessLog.AccessType.Write
            ) {
                // validate write proofs with old value, and update the machine hash with new value
                // developer should make sure the _oldHashes array matches the write operations in the _accesses array
                require(
                    _finalState.eq(
                        Machine.Hash.wrap(
                            Merkle.getRootWithHash(
                                _accessLogs.logs[_i].position,
                                _oldHashes[_writeCount],
                                _proofs[_i]
                            )
                        )
                    ),
                    "machine hash doesn't match"
                );

                _finalState = Machine.Hash.wrap(
                    Merkle.getRootWithValue(
                        _accessLogs.logs[_i].position,
                        _accessLogs.logs[_i].val,
                        _proofs[_i]
                    )
                );

                _writeCount++;
            } else {
                // validate read proofs
                require(
                    _finalState.eq(
                        Machine.Hash.wrap(
                            Merkle.getRootWithValue(
                                _accessLogs.logs[_i].position,
                                _accessLogs.logs[_i].val,
                                _proofs[_i]
                            )
                        )
                    ),
                    "machine hash doesn't match"
                );
            }
        }
        // call machine-solidity-step to replay accessLogs
        IUArchState.State memory _state = IUArchState.State(
            stateInterface,
            _accessLogs
        );
        stepInterface.step(_state);

        return _finalState;
    }
}
