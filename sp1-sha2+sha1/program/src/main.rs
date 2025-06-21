#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use hash_lib::{hash, PublicValuesStruct};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let data: Vec<u8> = sp1_zkvm::io::read_vec();

    let (sha1_hash, sha256_hash) = hash(data.clone());

    let public_values = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        size: data.len() as u32,
        sha1: alloy_primitives::FixedBytes(sha1_hash),
        sha2: alloy_primitives::FixedBytes(sha256_hash),
    });

    sp1_zkvm::io::commit(&public_values);
}
