use risc0_zkvm::guest::env;

use sha2::Digest;
use sha2::Sha256;
use sha1::Sha1;
use std::io::Read;

fn main() {
    let mut data = Vec::new();
    let len = env::stdin().read_to_end(&mut data).unwrap() as u64;
    
    env::commit_slice(&len.to_le_bytes());

    env::commit_slice(&Sha1::digest(&data));
    env::commit_slice(&Sha256::digest(&data));
}
