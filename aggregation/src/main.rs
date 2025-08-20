// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use sha2::Sha256;
// TODO: Replace optimized tiny keccak?
use sha3::{Digest as Sha3Digest, Keccak256};

pub fn main() {
    // Read the verification key. Same one for all proofs.
    let vkey = sp1_zkvm::io::read::<[u32; 8]>();

    // Read the public values.
    let public_values = sp1_zkvm::io::read::<Vec<Vec<u8>>>();
    // TODO: Maybe pass directly the hash of the public values

    // Verify the proofs.
    let mut keccak_chain = Keccak256::new();
    let mut pp_hashchain: [u8; 32] = [0u8; 32];

    for i in 0..public_values.len() {
        let public_values = &public_values[i];
        let public_values_digest = Sha256::digest(public_values);
        sp1_zkvm::lib::verify::verify_sp1_proof(&vkey, &public_values_digest.into());

        // Update keccak256 hashchain: H = keccak256(H || sha256_digest(public_values))
        keccak_chain.update(pp_hashchain);
        keccak_chain.update(public_values_digest.as_slice());
        pp_hashchain.copy_from_slice(&keccak_chain.finalize_reset());
    }

    sp1_zkvm::io::commit_slice(&pp_hashchain);
    sp1_zkvm::io::commit_slice(
        &vkey
            .iter()
            .flat_map(|w| w.to_be_bytes())
            .collect::<Vec<u8>>(),
    );
}
