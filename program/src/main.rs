//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use cairo_air::{verifier::verify_cairo, CairoProof, PreProcessedTraceVariant};
use stwo_prover::core::{
    pcs::PcsConfig,
    vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher},
};

pub fn main() {
    let (proof, pcs_config) = sp1_zkvm::io::read::<(CairoProof<Blake2sMerkleHasher>, PcsConfig)>();
    verify_cairo::<Blake2sMerkleChannel>(
        proof,
        pcs_config,
        PreProcessedTraceVariant::CanonicalWithoutPedersen,
    )
    .unwrap();
}
