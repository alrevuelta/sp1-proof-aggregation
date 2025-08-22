use agglayer_tries::{roots::LocalExitRoot, smt::Smt};
use alloy::{
    consensus::{SignableTransaction, TxEip1559, TypedTransaction},
    signers::{local::PrivateKeySigner, Signer},
};
use alloy_primitives::Address;
use alloy_primitives::{address, Signature, U256};
use alloy_sol_types::sol;
use clap::Parser;
use futures;
use pessimistic_proof::core::commitment::SignatureCommitmentValues;
use pessimistic_proof::local_balance_tree::LocalBalanceTree;
use pessimistic_proof::local_exit_tree::data::LocalExitTreeData;
use pessimistic_proof::local_state::LocalNetworkState;
use pessimistic_proof::nullifier_tree::{NullifierKey, NullifierPath, NullifierTree};
use pessimistic_proof::unified_bridge::{
    BridgeExit, Claim, ClaimFromMainnet, CommitmentVersion, GlobalIndex, ImportedBridgeExit,
    L1InfoTreeLeaf, L1InfoTreeLeafInner, LETMerkleProof, MerkleProof,
};
use pessimistic_proof::unified_bridge::{ImportedBridgeExitCommitmentValues, TokenInfo};
use pessimistic_proof::unified_bridge::{LeafType, LocalExitTree};
use pessimistic_proof::ELF as PESSIMISTIC_ELF;
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

use std::str::FromStr;
use std::{collections::BTreeMap, process::exit};

pub const AGGREGATION_ELF: &[u8] = include_elf!("aggregation-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    vkeys: bool,

    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long)]
    aggregate: bool,
}

#[tokio::main]
async fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute && args.prove && args.vkeys {
        eprintln!("Error: You must specify either --execute or --prove or --vkeys");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::from_env();

    let (aggregation_pk, aggregation_vk) = client.setup(AGGREGATION_ELF);
    let (pessimistic_pk, pessimistic_vk) = client.setup(PESSIMISTIC_ELF);

    if args.vkeys {
        println!("aggregation_vk: {:?}", aggregation_vk.bytes32());
        println!("pessimistic_vk: {:?}", pessimistic_vk.bytes32());
        let hash_bytes: Vec<u8> = pessimistic_vk
            .vk
            .hash_u32()
            .iter()
            .flat_map(|w| w.to_be_bytes())
            .collect();
        println!("pessimistic_vk_hash: 0x{}", hex::encode(hash_bytes));
        exit(0);
    }

    // Generate Pessimistic Proofs for two chains.
    let aggchain_ids = vec![0, 1];

    let initial_states: Vec<NetworkState> = aggchain_ids
        .iter()
        .map(|_i| LocalNetworkState::default().into())
        .collect();

    let batch_headers: Vec<MultiBatchHeader> = futures::future::join_all(
        initial_states
            .iter()
            .enumerate()
            .map(|(i, s)| create_batch_header(s, i as u32)),
    )
    .await;

    println!("aggregation_vk: {:?}", aggregation_vk.bytes32());
    println!("pessimistic_vk: {:?}", pessimistic_vk.bytes32());
    let hash_bytes: Vec<u8> = pessimistic_vk
        .vk
        .hash_u32()
        .iter()
        .flat_map(|w| w.to_be_bytes())
        .collect();
    println!("pessimistic_vk_hash: 0x{}", hex::encode(hash_bytes));

    if args.execute {
        for (initial_state, batch_header) in initial_states.iter().zip(batch_headers.iter()) {
            // Setup the inputs.
            let mut stdin = SP1Stdin::new();

            stdin.write(&initial_state);
            stdin.write(&batch_header);

            // Execute the program
            let (output, report) = client.execute(PESSIMISTIC_ELF, &stdin).run().unwrap();

            println!(
                "[aggchain_id: {}] Pessimistic proof program executed successfully.",
                batch_header.origin_network
            );

            // Read the output.
            let decoded: PessimisticProofOutput = PessimisticProofOutput::bincode_codec()
                .deserialize(output.as_slice())
                .unwrap();

            println!(
                "[aggchain_id: {}] Public values: {:?}",
                batch_header.origin_network, decoded
            );

            // Record the number of cycles executed.
            println!(
                "[aggchain_id: {}] Number of cycles: {}",
                batch_header.origin_network,
                report.total_instruction_count()
            );
        }
    } else {
        // Generate and store all pessimistic proofs for all aggchain_ids.
        let mut pp_proofs: Vec<SP1ProofWithPublicValues> = Vec::new();

        for (initial_state, batch_header) in initial_states.iter().zip(batch_headers.iter()) {
            // Setup the inputs.
            let mut stdin = SP1Stdin::new();

            stdin.write(&initial_state);
            stdin.write(&batch_header);

            // Generate the proof. If we want to aggregate, we use the compressed proof.
            let proof = if args.aggregate {
                client.prove(&pessimistic_pk, &stdin).compressed()
            } else {
                client.prove(&pessimistic_pk, &stdin).plonk()
            }
            .run()
            .expect("failed to generate proof");

            println!(
                "[aggchain_id: {}] Successfully generated proof!",
                batch_header.origin_network
            );

            // Verify the proof.
            client
                .verify(&proof, &pessimistic_vk)
                .expect("failed to verify proof");

            println!(
                "[aggchain_id: {}] Successfully verified proof!",
                batch_header.origin_network
            );

            if !args.aggregate {
                let proof_formated = format!("0x{}", hex::encode(proof.bytes()));

                println!(
                    "[aggchain_id: {}] Proof: {:?}",
                    batch_header.origin_network, proof_formated
                );
            }

            println!(
                "[aggchain_id: {}] Public values: {:?}",
                batch_header.origin_network,
                format!("0x{}", hex::encode(proof.clone().public_values.as_slice()))
            );

            pp_proofs.push(proof);
        }

        // Only generate the aggregation proof if the user wants to.
        if !args.aggregate {
            exit(0);
        }

        // Now we create a proof where we verify that the past proofs are correct.

        // Create a proof that verifies all the pessimistic proofs.
        let mut stdin = SP1Stdin::new();

        // Write the verification key.
        let vkey = pessimistic_vk.hash_u32();
        stdin.write::<[u32; 8]>(&vkey);

        // Write the public values.
        let public_values = pp_proofs
            .iter()
            .map(|proof| proof.public_values.to_vec())
            .collect::<Vec<_>>();
        stdin.write::<Vec<Vec<u8>>>(&public_values);

        // Write the proofs.
        // Note: this data will not actually be read by the aggregation program, instead it will be
        // witnessed by the prover during the recursive aggregation process inside SP1 itself.
        for input in pp_proofs {
            let SP1Proof::Compressed(proof) = input.proof else {
                panic!()
            };
            stdin.write_proof(*proof, pessimistic_vk.vk.clone());
        }

        // Generate the plonk bn254 proof.
        let proof = client
            .prove(&aggregation_pk, &stdin)
            .plonk()
            .run()
            .expect("proving failed");

        println!(
            "Successfully generated aggregation proof of aggchain_ids: {:?}",
            aggchain_ids
        );

        // Verify the proof.
        client
            .verify(&proof, &aggregation_vk)
            .expect("failed to verify proof");
        println!(
            "Successfully verified aggregation proof of aggchain_ids: {:?}",
            aggchain_ids
        );

        // Print the public inputs and the proof of the aggregated proof.
        let public_values = format!("0x{}", hex::encode(proof.clone().public_values.as_slice()));
        let proof_formated = format!("0x{}", hex::encode(proof.bytes()));

        println!("Aggregated proof public values: {:?}", public_values);
        println!("Aggregated proof: {:?}", proof_formated);
    }
}

// Util to create a bridge exit from one network to another.
fn exit_from_to(from: u32, to: u32) -> BridgeExit {
    BridgeExit {
        leaf_type: LeafType::Transfer,
        dest_network: to.into(),
        dest_address: Address::default().into(),
        token_info: TokenInfo {
            origin_network: from.into(),
            origin_token_address: Address::from_str("0xdead000000000000000000000000000000000000")
                .unwrap()
                .into(),
        },
        amount: U256::from(1000000000).into(),
        metadata: None,
    }
}

// Util to create a simple signed batch header with two bridge exits for a given network.
async fn create_batch_header(initial_state: &NetworkState, network_id: u32) -> MultiBatchHeader {
    // Some private key for signing
    let signer = PrivateKeySigner::from_slice(&[0x55; 32]).unwrap();

    // Create the batch header. Mostly empty
    let mut batch_header: MultiBatchHeader = MultiBatchHeader {
        origin_network: network_id.into(),
        height: 0,
        prev_pessimistic_root: B256::ZERO.into(),
        // Add a couple of bridge exits from origin to some destination.
        bridge_exits: vec![
            exit_from_to(network_id, 9999),
            exit_from_to(network_id, 9998),
        ],
        imported_bridge_exits: vec![],
        l1_info_root: B256::ZERO.into(),
        balances_proofs: BTreeMap::new(),
        aggchain_proof: AggchainData::ECDSA {
            // Default value, replaced later with the signature
            signer: Address::default().into(),
            // Default value, replaced later with the signature
            signature: Signature::from_raw_array(&[0x00; 65]).unwrap().into(),
        },
    };

    // Cloning since we dont want to mutate the initial state.
    let final_state_commitment = initial_state
        .clone()
        .apply_batch_header(&batch_header)
        .unwrap();

    let commitment = SignatureCommitmentValues {
        new_local_exit_root: final_state_commitment.exit_root,
        commit_imported_bridge_exits: ImportedBridgeExitCommitmentValues { claims: vec![] },
        height: 0,
    }
    .commitment(CommitmentVersion::V3);

    // Sign the commitment and add it to the batch header.
    let signed_commitment = signer.sign_hash(&commitment.into()).await.unwrap();
    batch_header.aggchain_proof = AggchainData::ECDSA {
        signer: signed_commitment
            .recover_address_from_prehash(&commitment.into())
            .unwrap()
            .into(),
        signature: signed_commitment.into(),
    };

    // Simulate the state transition
    let (pp_output, _) = generate_pessimistic_proof(initial_state.clone(), &batch_header).unwrap();
    println!(
        "[aggchain_id: {}] State transition LER: {:?} -> {:?}",
        network_id, pp_output.prev_local_exit_root, pp_output.new_local_exit_root
    );

    println!(
        "[aggchain_id: {}] State transition PPRoot: {:?} -> {:?}",
        network_id, pp_output.prev_pessimistic_root, pp_output.new_pessimistic_root
    );
    println!(
        "[aggchain_id: {}] Aggchain hash: {:?}",
        network_id, pp_output.aggchain_hash
    );
    println!(
        "[aggchain_id: {}] L1 info root: {:?}",
        network_id, pp_output.l1_info_root
    );

    batch_header
}
