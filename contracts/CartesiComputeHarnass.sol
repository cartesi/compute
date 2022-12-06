pragma solidity ^0.7.0;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/utils/Pausable.sol";
import "./CartesiComputeInterface.sol";

contract CartesiComputeHarnass is Ownable, Pausable {
    CartesiComputeInterface cc;

    constructor(address _cartesiCompute) {
        cc = CartesiComputeInterface(_cartesiCompute);
    }

    function instantiate(
        uint256 _finalTime,
        bytes32 _templateHash,
        uint64 _outputPosition,
        uint8 _outputLog2Size,
        uint256 _roundDuration,
        address[] memory parties,
        CartesiComputeInterface.Drive[] memory _inputDrives,
        bool _noChallengeDrive
    ) public whenNotPaused returns (uint256) {
        cc.instantiate(
            _finalTime,
            _templateHash,
            _outputPosition,
            _outputLog2Size,
            _roundDuration,
            parties,
            _inputDrives,
            _noChallengeDrive
        );
    }

    function pause() public onlyOwner {
        _pause();
    }

    function unpause() public onlyOwner {
        _unpause();
    }
}
