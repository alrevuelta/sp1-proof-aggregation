// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract VerifyPessimisticProof {
    address public verifier;
    bytes32 public programVKey;

    constructor(address _verifier, bytes32 _programVKey) {
        verifier = _verifier;
        programVKey = _programVKey;
    }

    function verifyPessimisticProof(bytes calldata _publicValues, bytes calldata _proofBytes)
        public
        view
    {
        ISP1Verifier(verifier).verifyProof(programVKey, _publicValues, _proofBytes);
    }

    function verifyGenericProof(
        bytes32 _vKey,
        bytes calldata _publicValues,
        bytes calldata _proofBytes
    ) public view returns (bool success, bytes memory errorData) {
        try ISP1Verifier(verifier).verifyProof(_vKey, _publicValues, _proofBytes) {
            return (true, "");
        } catch Error(string memory reason) {
            return (false, bytes(reason));
        } catch (bytes memory lowLevelData) {
            return (false, lowLevelData);
        }
    }

    function getVKey() public view returns (bytes32) {
        return programVKey;
    }
}
