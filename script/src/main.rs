//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{ProverClient, SP1Stdin};
use tiny_keccak::{Hasher, Sha3};
use rand::Rng;
use std::time::Instant;

const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    let mut byte_vec = Vec::new();
    let n = 1024;
    for _ in 0..n {
        let mut rand_bytes = [0u8; 32];
        rand::thread_rng().fill(&mut rand_bytes);
        byte_vec.push(rand_bytes);
    }

    // Generate proof.
    let mut stdin = SP1Stdin::new();
    stdin.write(&byte_vec);
    let client = ProverClient::new();
    let (pk, vk) = client.setup(ELF);
    println!("Building proof...");
    let start_time = Instant::now();
    let mut proof = client.prove(&pk, stdin).expect("proving failed");
    let prove_time = Instant::now() - start_time;
    println!("Proof done in {}s", prove_time.as_secs());
    // Read output.
    let a = proof.public_values.read::<[u8; 32]>();
    println!("a: {:?}", a);

    let mut sha3 = Sha3::v256();
    let mut output_truth = [0u8; 32];
    byte_vec.iter().for_each(|b| {sha3.update(b)});
    sha3.finalize(&mut output_truth);
    assert_eq!(a, output_truth);

    // Verify proof.
    client.verify(&proof, &vk).expect("verification failed");

    // Save proof.
    proof
        .save("proof-with-io.json")
        .expect("saving proof failed");

    println!("successfully generated and verified proof for the program!")
}
