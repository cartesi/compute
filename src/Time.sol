// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

library Time {
    type Instant is uint64;
    type Duration is uint64;

    using Time for Instant; // TODO rename to Instant
    using Time for Duration;

    Instant constant ZERO_INSTANT = Instant.wrap(0);
    Duration constant ZERO_DURATION = Duration.wrap(0);

    function currentTime() internal view returns (Instant) {
        return Instant.wrap(uint64(block.timestamp));
    }

    function add(
        Instant timestamp,
        Duration duration
    ) internal pure returns (Instant) {
        uint64 t = Instant.unwrap(timestamp);
        uint64 d = Duration.unwrap(duration);
        return Instant.wrap(t + d);
    }

    function gt(Instant left, Instant right) internal pure returns (bool) {
        uint64 l = Instant.unwrap(left);
        uint64 r = Instant.unwrap(right);
        return l > r;
    }

    function gt(Duration left, Duration right) internal pure returns (bool) {
        uint64 l = Duration.unwrap(left);
        uint64 r = Duration.unwrap(right);
        return l > r;
    }

    function isZero(Instant timestamp) internal pure returns (bool) {
        uint64 t = Instant.unwrap(timestamp);
        return t == 0;
    }

    function isZero(Duration duration) internal pure returns (bool) {
        uint64 d = Duration.unwrap(duration);
        return d == 0;
    }

    function add(
        Duration left,
        Duration right
    ) internal pure returns (Duration) {
        uint64 l = Duration.unwrap(left);
        uint64 r = Duration.unwrap(right);
        return Duration.wrap(l + r);
    }

    function sub(
        Duration left,
        Duration right
    ) internal pure returns (Duration) {
        uint64 l = Duration.unwrap(left);
        uint64 r = Duration.unwrap(right);
        return Duration.wrap(l - r);
    }

    function monus(
        Duration left,
        Duration right
    ) internal pure returns (Duration) {
        uint64 l = Duration.unwrap(left);
        uint64 r = Duration.unwrap(right);
        return Duration.wrap(l < r ? 0 : l - r);
    }

    function timeSpan(
        Instant left,
        Instant right
    ) internal pure returns (Duration) {
        uint64 l = Instant.unwrap(left);
        uint64 r = Instant.unwrap(right);
        return Duration.wrap(l - r);
    }

    function timeoutElapsedSince(
        Instant timestamp,
        Duration duration,
        Instant current
    ) internal pure returns (bool) {
        return !timestamp.add(duration).gt(current);
    }

    function timeoutElapsed(
        Instant timestamp,
        Duration duration
    ) internal view returns (bool) {
        return timestamp.timeoutElapsedSince(duration, currentTime());
    }
}
