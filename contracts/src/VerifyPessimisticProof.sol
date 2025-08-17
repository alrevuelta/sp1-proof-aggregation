// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

contract VerifyPessimisticProof {
    address public verifier;
    mapping(uint32 => bytes32) public hashMapping;

    constructor(address _verifier) {
        verifier = _verifier;
    }

    function getVerifier() public view returns (address) {
        return verifier;
    }

    function verifyPessimisticProof(bytes32 _vKey, bytes calldata _publicValues, bytes calldata _proofBytes)
        public
    {
        // Do stuff
        ISP1Verifier(verifier).verifyProof(_vKey, _publicValues, _proofBytes);

        // TODO: Decode parameters of aggchainid and new LER and store them in the mapping.
        hashMapping[0] = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    }

    function verifyMultiplePessimisticProofs(bytes32 _vKey, bytes calldata _publicValues, bytes calldata _proofBytes)
        public
    {
        ISP1Verifier(verifier).verifyProof(_vKey, _publicValues, _proofBytes);

        // TODO: Decode parameters of aggchainid and new LER and store them in the mapping.
        hashMapping[0] = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    }
}
