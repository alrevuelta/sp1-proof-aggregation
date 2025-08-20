// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// TODO:

import {Test, console} from "forge-std/Test.sol";
import {stdJson} from "forge-std/StdJson.sol";
import {VerifyPessimisticProof} from "../src/VerifyPessimisticProof.sol";
import {PessimisticProofOutput} from "../src/VerifyPessimisticProof.sol";
import {SP1VerifierGateway} from "@sp1-contracts/SP1VerifierGateway.sol";


struct SP1ProofFixtureJson {
    uint32 a;
    uint32 b;
    uint32 n;
    bytes proof;
    bytes publicValues;
    bytes32 vkey;
}


contract VerifyPessimisticProofTest is Test {
    using stdJson for string;

    address verifier;
    VerifyPessimisticProof public verifyPessimisticProof;

    /*
    function loadFixture() public view returns (SP1ProofFixtureJson memory) {
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/src/fixtures/plonk-fixture.json");
        string memory json = vm.readFile(path);
        bytes memory jsonBytes = json.parseRaw(".");
        return abi.decode(jsonBytes, (SP1ProofFixtureJson));
    }*/

    function setUp() public {
        //SP1ProofFixtureJson memory fixture = loadFixture();

        verifier = address(new SP1VerifierGateway(address(1)));
        verifyPessimisticProof = new VerifyPessimisticProof(verifier);
    }

    function test_VerifyPessimisticProof() public {
        // TODO
    }

    function test_computeHashChain() public {
        bytes[] memory ppOutputs = new bytes[](2);
        ppOutputs[0] = hex"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003f4b9e2bb63b8a124ca9c44e465dadff6605b3b728a63876df1bc8848fedb70956e89bd824cc7d85242ce8d71551116030feb1cd7d5794f56af492fa0c4a8d7e78392453e02202258713d73205a965fe8028cdba5af92208fd500a58696372e5";
        ppOutputs[1] = hex"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000013f4b9e2bb63b8a124ca9c44e465dadff6605b3b728a63876df1bc8848fedb70913172e09840453518fe8fe3b5174c1d4ebe3614a492596569acd8c012839135e77a4e3ccd788020f5b86128c69d846faa47e54c2f4e1d85be6020ad75271f173";

        bytes32 hashChain = verifyPessimisticProof.computeHashChain(ppOutputs);
        assert(hashChain == 0x5bb178b0da10539c84d2304def6d6ddc26f741e3b80f5f994c8fc87029bbaac2);
    }

    function test_decodePublicValues() public {
        bytes memory ppOutput = hex"000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000013f4b9e2bb63b8a124ca9c44e465dadff6605b3b728a63876df1bc8848fedb70913172e09840453518fe8fe3b5174c1d4ebe3614a492596569acd8c012839135e77a4e3ccd788020f5b86128c69d846faa47e54c2f4e1d85be6020ad75271f173";
        PessimisticProofOutput memory decoded = verifyPessimisticProof.decodePublicValues(ppOutput);

        assert(decoded.prev_local_exit_root == hex"0000000000000000000000000000000000000000000000000000000000000000");
        assert(decoded.prev_pessimistic_root == hex"0000000000000000000000000000000000000000000000000000000000000000");
        assert(decoded.l1_info_root == hex"0000000000000000000000000000000000000000000000000000000000000000");
        assert(decoded.origin_network == 1);
        assert(decoded.aggchain_hash == hex"3f4b9e2bb63b8a124ca9c44e465dadff6605b3b728a63876df1bc8848fedb709");
        assert(decoded.new_local_exit_root == hex"13172e09840453518fe8fe3b5174c1d4ebe3614a492596569acd8c012839135e");
        assert(decoded.new_pessimistic_root == hex"77a4e3ccd788020f5b86128c69d846faa47e54c2f4e1d85be6020ad75271f173");
    }
/*
    function test_ValidFibonacciProof() public {
        SP1ProofFixtureJson memory fixture = loadFixture();

        vm.mockCall(verifier, abi.encodeWithSelector(SP1VerifierGateway.verifyProof.selector), abi.encode(true));

        (uint32 n, uint32 a, uint32 b) = fibonacci.verifyFibonacciProof(fixture.publicValues, fixture.proof);
        assert(n == fixture.n);
        assert(a == fixture.a);
        assert(b == fixture.b);
    }

    function testRevert_InvalidFibonacciProof() public {
        vm.expectRevert();

        SP1ProofFixtureJson memory fixture = loadFixture();

        // Create a fake proof.
        bytes memory fakeProof = new bytes(fixture.proof.length);

        fibonacci.verifyFibonacciProof(fixture.publicValues, fakeProof);
    }
    */
}
