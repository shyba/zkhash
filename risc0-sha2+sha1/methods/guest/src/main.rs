use risc0_zkvm::guest::env;

use sha2::Digest;
use sha2::Sha256;
use sha1::Sha1;

fn main() {
    let data: Vec<u8> = env::read();
    
    let data_size = data.len() as u64;
    env::commit_slice(&data_size.to_le_bytes());

    let mut hasher = Sha1::new();
    hasher.update(&data);
    let sha1_hash: [u8; 20] = hasher.finalize().into();
    env::commit_slice(&sha1_hash);
    
    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256_hash: [u8; 32] = hasher.finalize().into();
    env::commit_slice(&sha256_hash);
}
