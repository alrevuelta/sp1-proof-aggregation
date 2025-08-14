//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_primitives::{address, Signature, U256};
//use alloy_sol_types::sol_data::Address;
use agglayer_tries::{roots::LocalExitRoot, smt::Smt};
use agglayer_types::LocalNetworkStateData;
use alloy_primitives::Address;
use alloy_sol_types::SolType;
use clap::Parser;
use pessimistic_proof::local_exit_tree::data::LocalExitTreeData;
//use hex_literal::hex;
use alloy::{
    consensus::{SignableTransaction, TxEip1559, TypedTransaction},
    signers::{local::PrivateKeySigner, Signer},
};
use pessimistic_proof::core::commitment::SignatureCommitmentValues;
use pessimistic_proof::local_balance_tree::LocalBalanceTree;
use pessimistic_proof::local_state::LocalNetworkState;
use pessimistic_proof::unified_bridge::{
    BridgeExit, Claim, ClaimFromMainnet, CommitmentVersion, GlobalIndex, ImportedBridgeExit,
    L1InfoTreeLeaf, L1InfoTreeLeafInner, LETMerkleProof, MerkleProof,
};
use pessimistic_proof::unified_bridge::{ImportedBridgeExitCommitmentValues, TokenInfo};
use pessimistic_proof::unified_bridge::{LeafType, LocalExitTree};
use std::str::FromStr;
use std::{collections::BTreeMap, process::exit};
//use pessimistic_proof_core::local_balance_tree::LocalBalanceTree;
//use pessimistic_proof_core::local_balance_tree::LocalBalanceTree;

use pessimistic_proof::nullifier_tree::{NullifierKey, NullifierPath, NullifierTree};
use pessimistic_proof::ELF as FIBONACCI_ELF;
use pessimistic_proof_core::aggchain_proof::AggchainData;
use pessimistic_proof_core::{
    generate_pessimistic_proof, multi_batch_header::MultiBatchHeader, NetworkState,
    PessimisticProofOutput,
};
use sp1_sdk::network::B256;
use sp1_sdk::{
    include_elf, HashableKey, ProverClient, SP1Proof, SP1ProofWithPublicValues, SP1Stdin,
    SP1VerifyingKey,
};

pub const AGGREGATION_ELF: &[u8] = include_elf!("aggregation-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "20")]
    n: u32,
}

struct AggregationInput {
    pub proof: SP1ProofWithPublicValues,
    pub vk: SP1VerifyingKey,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    let initial_state: NetworkState = LocalNetworkState {
        exit_tree: LocalExitTree::default(),
        balance_tree: LocalBalanceTree::new(),
        nullifier_tree: NullifierTree::new(),
    }
    .into();

    let private_key = [0x55; 32];
    let signer = PrivateKeySigner::from_slice(&private_key).unwrap();

    println!("initial_state: {:?}", initial_state);

    println!(
        "------new ler default------: {:?}",
        LocalExitRoot::default()
    );

    let exits = vec![BridgeExit {
        leaf_type: LeafType::Transfer,
        dest_network: 1.into(),
        dest_address: Address::default().into(),
        token_info: TokenInfo {
            origin_network: 0.into(),
            origin_token_address: Address::default().into(),
        },
        amount: U256::from(1000000000).into(),
        metadata: None,
    }];

    //let exits = vec![];
    /*
    let letmerkleproof = LETMerkleProof {
        siblings: [B256::ZERO; 32],
    }; */

    let mut hola = LocalExitTreeData::new();
    hola.add_leaf(B256::ZERO.into());

    let imported_exits = vec![ImportedBridgeExit {
        bridge_exit: BridgeExit {
            // TODO: play around with origin and destination here.
            leaf_type: LeafType::Transfer,
            dest_network: 1.into(),
            dest_address: Address::default().into(),
            token_info: TokenInfo {
                origin_network: 0.into(),
                origin_token_address: Address::default().into(),
            },
            amount: U256::from(1000000000).into(),
            metadata: None,
        },
        claim_data: Claim::Mainnet(Box::new(ClaimFromMainnet {
            proof_leaf_mer: MerkleProof {
                proof: hola.get_proof(0).unwrap(),
                root: hola.get_root(),
            },
            proof_ger_l1root: MerkleProof {
                proof: hola.get_proof(0).unwrap(),
                root: hola.get_root(),
            },
            l1_leaf: L1InfoTreeLeaf {
                l1_info_tree_index: 0,
                rer: B256::ZERO.into(),
                mer: B256::ZERO.into(),
                inner: L1InfoTreeLeafInner {
                    global_exit_root: B256::ZERO.into(),
                    block_hash: B256::ZERO.into(),
                    timestamp: 0,
                },
            },
        })),
        global_index: GlobalIndex::new(0.into(), 0),
    }];
    /*
    let imported_bridge_exits: Vec<(ImportedBridgeExit, NullifierPath)> = imported_exits
        .iter()
        .map(|exit| {
            let nullifier_key: NullifierKey = exit.global_index.into();
            let nullifier_error = |source| Error::NullifierPathGenerationFailed {
                source,
                global_index: exit.global_index,
            };
            let nullifier_path = initial_state
                .nullifier_tree
                .get_non_inclusion_proof(nullifier_key)
                .map_err(nullifier_error)?;
            initial_state
                .nullifier_tree
                .insert(nullifier_key, Digest::from_bool(true))
                .map_err(nullifier_error)?;
            Ok((exit.clone(), nullifier_path))
        })
        .collect::<Result<Vec<_>, Error>>()?;
     */

    let mut batch_header: MultiBatchHeader = MultiBatchHeader {
        origin_network: 0.into(),
        height: 0,
        prev_pessimistic_root: B256::ZERO.into(),
        bridge_exits: exits,
        imported_bridge_exits: vec![],
        l1_info_root: B256::ZERO.into(),
        balances_proofs: BTreeMap::new(),
        aggchain_proof: AggchainData::ECDSA {
            signer: Address::default().into(),
            // random number, this is replaced.
            signature: Signature::from_str("48b55bfa915ac795c431978d8a6a992b628d557da5ff759b307d495a36649353efffd310ac743f371de3b9f7f9cb56c0b28ad43601b4ab949f53faa07bd2c8041b").unwrap().into(),
        },
    };
    println!("batch_header: {:?}", batch_header);

    let mut cloned_initial_state = initial_state.clone();

    let final_state_commitment = cloned_initial_state
        .apply_batch_header(&batch_header)
        .unwrap();

    println!("count: {:?}", cloned_initial_state.exit_tree.leaf_count);

    let signature_commitment_values = SignatureCommitmentValues {
        new_local_exit_root: final_state_commitment.exit_root,
        // TODO:? I have to modify this if adding imports?
        commit_imported_bridge_exits: ImportedBridgeExitCommitmentValues { claims: vec![] },
        height: 0,
    };

    let commitment = signature_commitment_values.commitment(CommitmentVersion::V3);
    println!("commitment: {:?}", commitment);
    let signed_commitment = signer.sign_hash(&commitment.into()).await.unwrap();
    println!("signed_commitment: {:?}", signed_commitment);
    let recovered_address = signed_commitment
        .recover_address_from_prehash(&commitment.into())
        .unwrap();
    println!("recovered_address: {:?}", recovered_address);
    // put the new signature here
    batch_header.aggchain_proof = AggchainData::ECDSA {
        signer: recovered_address.into(),
        signature: signed_commitment.into(),
    };

    println!(
        "final_state_commitment: {:?}",
        final_state_commitment.exit_root
    );

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    //stdin.write(&args.n);
    stdin.write(&initial_state);
    stdin.write(&batch_header);

    println!("n: {}", args.n);
    let (aggregation_pk, aggregation_vk) = client.setup(AGGREGATION_ELF);
    let (input_pk, input_vk) = client.setup(FIBONACCI_ELF);

    println!("aggregation_vk: {:?}", aggregation_vk.bytes32());
    println!("input_vk: {:?}", input_vk.bytes32());

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(FIBONACCI_ELF, &stdin).run().unwrap();

        println!("Program executed successfully.");
        // Proof aggregation execution. not working

        // Read the output.
        let decoded: PessimisticProofOutput = PessimisticProofOutput::bincode_codec()
            .deserialize(output.as_slice())
            .unwrap();
        println!("decoded: {:?}", decoded);
        /*
               //let PessimisticProofOutput { n, a, b } = decoded;

               let input_1 = AggregationInput {
                   proof: output,
                   vk: input_vk.clone(),
               };
               // TODO: Repeated for now.
               let inputs = vec![input_1, input_1, input_1];

               let mut stdin = SP1Stdin::new();

               // Write the verification keys.
               let vkeys = inputs
                   .iter()
                   .map(|input| input.vk.hash_u32())
                   .collect::<Vec<_>>();
               stdin.write::<Vec<[u32; 8]>>(&vkeys);

               // Write the public values.
               let public_values = inputs
                   .iter()
                   .map(|input| input.proof.public_values.to_vec())
                   .collect::<Vec<_>>();
               stdin.write::<Vec<Vec<u8>>>(&public_values);

               // Write the proofs.
               //
               // Note: this data will not actually be read by the aggregation program, instead it will be
               // witnessed by the prover during the recursive aggregation process inside SP1 itself.
               for input in inputs {
                   let SP1Proof::Compressed(proof) = input.proof.proof else {
                       panic!()
                   };
                   stdin.write_proof(*proof, input.vk.vk);
               }

               // Generate the plonk bn254 proof.
               client
                   .prove(&aggregation_pk, &stdin)
                   .plonk()
                   .run()
                   .expect("proving failed");
        */
        /*
        let (expected_a, expected_b) = fibonacci_lib::fibonacci(n);
        assert_eq!(a, expected_a);
        assert_eq!(b, expected_b);
        println!("Values are correct!"); */

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(FIBONACCI_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .plonk()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");

        let public_values = format!("0x{}", hex::encode(proof.clone().public_values.as_slice()));
        let proof_formated = format!("0x{}", hex::encode(proof.bytes()));
        // TODO: unsure if the proof i have to submite the proof or proof.proof. the first proof
        // contains the public values as well. it would not make sense but the fibonaci example uses that.

        println!("public_values: {:?}", public_values);
        println!("proof: {:?}", proof_formated);

        println!("not generating the aggregated proof");
        exit(0);

        let mut stdin = SP1Stdin::new();
        let input_1 = AggregationInput {
            proof,
            vk: input_vk.clone(),
        };
        // TODO: Repeated for now.
        let inputs = vec![input_1];

        // Write the verification keys.
        let vkeys = inputs
            .iter()
            .map(|input| input.vk.vk.hash_u32())
            .collect::<Vec<_>>();
        stdin.write::<Vec<[u32; 8]>>(&vkeys);

        // Write the public values.
        let public_values = inputs
            .iter()
            .map(|input| input.proof.public_values.to_vec())
            .collect::<Vec<_>>();
        stdin.write::<Vec<Vec<u8>>>(&public_values);

        // Write the proofs.
        //
        // Note: this data will not actually be read by the aggregation program, instead it will be
        // witnessed by the prover during the recursive aggregation process inside SP1 itself.
        for input in inputs {
            let SP1Proof::Compressed(proof) = input.proof.proof else {
                panic!()
            };
            stdin.write_proof(*proof, input.vk.vk);
        }

        // Generate the plonk bn254 proof.
        let proof = client
            .prove(&aggregation_pk, &stdin)
            .plonk()
            .run()
            .expect("proving failed");

        println!("Successfully generated proof!");

        // Verify the proof.
        client
            .verify(&proof, &aggregation_vk)
            .expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
