// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import {ISP1Verifier} from "@sp1-contracts/ISP1Verifier.sol";

struct PublicValuesStruct {
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

    // Some dummy implementation of the proof verification. Not aiming to be complete.
    function verifyPessimisticProof(bytes32 _vKey, bytes calldata _publicValues, bytes calldata _proofBytes)
        public
    {
        // Verify the Pessimistic Proof.
        ISP1Verifier(verifier).verifyProof(_vKey, _publicValues, _proofBytes);

        PublicValuesStruct memory publicValues = decodePublicValues(_publicValues);

        require(publicValues.prev_local_exit_root == aggchainToLER[publicValues.origin_network], "Invalid previous LER");
        require(publicValues.prev_pessimistic_root == aggchainToPPRoot[publicValues.origin_network], "Invalid previous PPRoot");

        // Do the state transition
        aggchainToLER[publicValues.origin_network] = publicValues.new_local_exit_root;
        aggchainToPPRoot[publicValues.origin_network] = publicValues.new_pessimistic_root;
    }

    function verifyMultiplePessimisticProofs(bytes32 _vKey, bytes calldata _publicValues, bytes calldata _proofBytes)
        public
    {
        ISP1Verifier(verifier).verifyProof(_vKey, _publicValues, _proofBytes);

        // TODO: Decode parameters of aggchainid and new LER and store them in the mapping.
        aggchainToLER[0] = 0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef;
    }

    function getLER(uint32 _aggchainId) public view returns (bytes32) {
        return aggchainToLER[_aggchainId];
    }
    function getPPRoot(uint32 _aggchainId) public view returns (bytes32) {
        return aggchainToPPRoot[_aggchainId];
    }

    // TODO: Tests this
    // Decodes the values into a struct. Abi decode can't be used since it was encoded with packed encoding.
    function decodePublicValues(bytes memory data)
        public
        pure
        returns (PublicValuesStruct memory s)
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
