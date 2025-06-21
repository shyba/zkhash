use alloy_sol_types::sol;
use sha1::{Digest, Sha1};
use sha2::Sha256;

sol! {
    struct PublicValuesStruct {
        uint32 size;
        bytes20 sha1;
        bytes32 sha2;
    }
}

pub fn hash(data: Vec<u8>) -> ([u8; 20], [u8; 32]) {
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let sha1_hash: [u8; 20] = hasher.finalize().into();

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let sha256_hash: [u8; 32] = hasher.finalize().into();
    (sha1_hash, sha256_hash)
}
