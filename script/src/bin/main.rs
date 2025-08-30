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

use std::{fs, path::PathBuf};

use cairo_air::CairoProof;
use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use stwo_prover::core::pcs::PcsConfig;
use stwo_prover::core::vcs::blake2_merkle::Blake2sMerkleHasher;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const STWO_VERIFIER_ELF: &[u8] = include_elf!("stwo-verifier");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    proof_path: PathBuf,

    #[clap(long)]
    config_path: PathBuf,
}

fn main() {
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

    let proof: CairoProof<Blake2sMerkleHasher> =
        serde_json::from_slice(&fs::read(args.proof_path).unwrap()).unwrap();

    let pcs_config: PcsConfig =
        serde_json::from_slice(&fs::read(args.config_path).unwrap()).unwrap();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&(proof, pcs_config));

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(STWO_VERIFIER_ELF, &stdin).run().unwrap();
        println!("Program executed successfully. {output:?}");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(STWO_VERIFIER_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .groth16()
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        println!("{proof:?}");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
