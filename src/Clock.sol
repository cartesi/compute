// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./Time.sol";
import "./CanonicalConstants.sol";

library Clock {
    using Time for Time.Instant;
    using Time for Time.Duration;

    using Clock for State;

    struct State {
        Time.Duration allowance;
        Time.Instant startInstant; // the timestamp when the clock started ticking, zero means clock is paused
    }

    //
    // View/Pure methods
    //

    function notInitialized(State memory state) internal pure returns (bool) {
        return state.allowance.isZero();
    }

    function requireInitialized(State memory state) internal pure {
        require(!state.notInitialized(), "clock is not initialized");
    }

    function requireNotInitialized(State memory state) internal pure {
        require(state.notInitialized(), "clock is initialized");
    }

    function hasTimeLeft(State memory state) internal view returns (bool) {
        if (state.startInstant.isZero()) {
            return true;
        } else {
            return
                state.allowance.gt(
                    Time.timeSpan(Time.currentTime(), state.startInstant)
                );
        }
    }

    /// @return deadline of the two clocks should be the tolerances combined
    function deadline(
        State memory freshState1,
        State memory freshState2
    ) internal view returns (Time.Instant) {
        Time.Duration duration = freshState1.allowance.add(
            freshState2.allowance
        );
        return Time.currentTime().add(duration);
    }

    /// @return max tolerance of the two clocks
    function max(
        State memory pausedState1,
        State memory pausedState2
    ) internal pure returns (Time.Duration) {
        if (pausedState1.allowance.gt(pausedState2.allowance)) {
            return pausedState1.allowance;
        } else {
            return pausedState2.allowance;
        }
    }

    /// @return duration of time has elapsed since the clock timeout
    function timeSinceTimeout(
        State memory state
    ) internal view returns (Time.Duration) {
        return
            Time.timeSpan(Time.currentTime(), state.startInstant).monus(
                state.allowance
            );
    }

    //
    // Storage methods
    //

    function setNewPaused(
        State storage state,
        Time.Instant checkinInstant,
        Time.Duration initialAllowance
    ) internal {
        Time.Duration allowance = initialAllowance.monus(
            Time.currentTime().timeSpan(checkinInstant)
        );

        if (allowance.isZero()) {
            revert("can't create clock with zero time");
        }

        state.allowance = allowance;
        state.startInstant = Time.ZERO_INSTANT;
    }

    /// @notice Resume the clock from pause state, or pause a clock and update the allowance
    function advanceClock(State storage state) internal {
        Time.Duration _timeLeft = timeLeft(state);

        if (_timeLeft.isZero()) {
            revert("can't advance clock with no time left");
        }

        toggleClock(state);
        state.allowance = _timeLeft;
    }

    function addValidatorEffort(State storage state, Time.Duration deduction) internal {
        Time.Duration _timeLeft = state.allowance.monus(
            deduction
        );

        if (_timeLeft.isZero()) {
            revert("can't reset clock with no time left");
        }

        Time.Duration _allowance = _timeLeft.add(ArbitrationConstants.VALIDATOR_EFFORT);
        if (_allowance.gt(ArbitrationConstants.DISPUTE_TIMEOUT)) {
            _allowance = ArbitrationConstants.DISPUTE_TIMEOUT;
        }

        state.allowance = _allowance;
        state.startInstant = Time.ZERO_INSTANT;
    }

    function setPaused(State storage state) internal {
        if (!state.startInstant.isZero()) {
            state.advanceClock();
        }
    }

    //
    // Private
    //

    function timeLeft(State memory state) private view returns (Time.Duration) {
        if (state.startInstant.isZero()) {
            return state.allowance;
        } else {
            return
                state.allowance.monus(
                    Time.timeSpan(Time.currentTime(), state.startInstant)
                );
        }
    }

    function toggleClock(State storage state) private {
        if (state.startInstant.isZero()) {
            state.startInstant = Time.currentTime();
        } else {
            state.startInstant = Time.ZERO_INSTANT;
        }
    }
}
