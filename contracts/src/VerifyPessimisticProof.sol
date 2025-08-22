// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

struct PessimisticProofOutput {
    bytes32 prev_local_exit_root;
    bytes32 prev_pessimistic_root;
    bytes32 l1_info_root;
    uint32 origin_network;
    bytes32 aggchain_hash;
    bytes32 new_local_exit_root;
    bytes32 new_pessimistic_root;
}

contract VerifyPessimisticProof {
    address public verifier;
    mapping(uint32 => bytes32) public aggchainToLER;
    mapping(uint32 => bytes32) public aggchainToPPRoot;

    constructor(address _verifier) {
        verifier = _verifier;
    }

    function getVerifier() public view returns (address) {
        return verifier;
    }

    // A dummy implementation of the proof verification. Not aiming to be complete.
    function verifyPessimisticProof(bytes32 _vKey, bytes calldata _ppOutput, bytes calldata _proofBytes)
        public
    {
        // Verify the Pessimistic Proof.
        ISP1Verifier(verifier).verifyProof(_vKey, _ppOutput, _proofBytes);

        PessimisticProofOutput memory ppOutput = decodePublicValues(_ppOutput);

        //require(ppOutput.prev_local_exit_root == aggchainToLER[ppOutput.origin_network], "Invalid previous LER");
        //require(ppOutput.prev_pessimistic_root == aggchainToPPRoot[ppOutput.origin_network], "Invalid previous PPRoot");

        // Do the state transition
        aggchainToLER[ppOutput.origin_network] = ppOutput.new_local_exit_root;
        aggchainToPPRoot[ppOutput.origin_network] = ppOutput.new_pessimistic_root;
    }
    
    // A dummy implementation to verify multiple proofs at once. Not aiming to be complete.
    function verifyMultiplePessimisticProofs(
        bytes32 _aggregationVKey,
        bytes32 _hashedPessimisticVKey,
        bytes[] calldata _ppOutputs,
        bytes calldata _proofBytes)
        public
    {
        // Verify the aggregated proof is correct.
        ISP1Verifier(verifier).verifyProof(
            _aggregationVKey,
            abi.encodePacked(computeHashChain(_ppOutputs), _hashedPessimisticVKey),
            _proofBytes);

        // Do the state transition for each aggchain.
        for (uint256 i = 0; i < _ppOutputs.length; i++) {
            PessimisticProofOutput memory ppOutput = decodePublicValues(_ppOutputs[i]);
            
            //require(ppOutput.prev_local_exit_root == aggchainToLER[ppOutput.origin_network], "Invalid previous LER");
            //require(ppOutput.prev_pessimistic_root == aggchainToPPRoot[ppOutput.origin_network], "Invalid previous PPRoot");

            // Do the state transition
            aggchainToLER[ppOutput.origin_network] = ppOutput.new_local_exit_root;
            aggchainToPPRoot[ppOutput.origin_network] = ppOutput.new_pessimistic_root;
        }
    }

    function getLER(uint32 _aggchainId) public view returns (bytes32) {
        return aggchainToLER[_aggchainId];
    }
    function getPPRoot(uint32 _aggchainId) public view returns (bytes32) {
        return aggchainToPPRoot[_aggchainId];
    }

    function computeHashChain(bytes[] calldata _ppOutputs) public pure returns (bytes32) {
        bytes32 hashChain = bytes32(0);
        for (uint256 i = 0; i < _ppOutputs.length; i++) {
            bytes32 ppOutputDigest = sha256(_ppOutputs[i]);
            hashChain = keccak256(abi.encodePacked(hashChain, ppOutputDigest));
        }
        return hashChain;
    }

    // Decodes the values into a struct. Abi decode can't be used since it was encoded with packed encoding.
    function decodePublicValues(bytes memory data)
        public
        pure
        returns (PessimisticProofOutput memory s)
    {
        require(data.length == 196, "PackedDecoder: invalid length");
        assembly {
            // Load first three bytes32
            s := mload(0x40) // allocate
            mstore(0x40, add(s, 0xE0)) // move free memory pointer (7 * 32 bytes)

            mstore(add(s, 0x00), mload(add(data, 32)))   // prev_local_exit_root
            mstore(add(s, 0x20), mload(add(data, 64)))   // prev_pessimistic_root
            mstore(add(s, 0x40), mload(add(data, 96)))   // l1_info_root

            // origin_network (4 bytes) — right-aligned into uint32
            let word := mload(add(data, 128))
            let shifted := shr(224, word) // take top 4 bytes
            mstore(add(s, 0x60), shifted)

            // remaining three bytes32
            mstore(add(s, 0x80), mload(add(data, 132)))  // aggchain_hash
            mstore(add(s, 0xA0), mload(add(data, 164)))  // new_local_exit_root
            mstore(add(s, 0xC0), mload(add(data, 196)))  // new_pessimistic_root
        }
    }
}
