//! A simple program to be proven inside the zkVM.

#![no_main]
sp1_zkvm::entrypoint!(main);


use tiny_keccak::{Hasher, Sha3};

pub fn main() {
    // NOTE: values of n larger than 186 will overflow the u128 type,
    // resulting in output that doesn't match fibonacci sequence.
    // However, the resulting proof will still be valid!
    let byte_vec = sp1_zkvm::io::read::<Vec<[u8; 32]>>();
    let r = challenge(byte_vec);
    sp1_zkvm::io::commit(&r);
}

pub fn challenge (
    byte_vec: Vec<[u8; 32]>,
    ) -> [u8; 32] {
    let mut sha3 = Sha3::v256();
    let mut output = [0; 32];
    byte_vec.iter().for_each(|b|{ sha3.update(b) });
    
    sha3.finalize(&mut output);
    output
}
