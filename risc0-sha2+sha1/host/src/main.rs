use anyhow::Result;
use clap::{Parser, Subcommand};
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};
use serde::{Deserialize, Serialize};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::Sha256;
use std::io::{self, Read, Write};

use methods::{HASHER_ELF, HASHER_ID};

#[derive(Parser)]
#[command(name = "hashapp")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Prove,
    Verify,
}

#[derive(Serialize, Deserialize)]
struct HashProof {
    receipt: Receipt,
    sha1_hash: [u8; 20],
    sha256_hash: [u8; 32],
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Prove => prove(),
        Commands::Verify => verify(),
    }
}

fn prove() -> Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let mut sha1_hasher = Sha1::new();
    sha1_hasher.update(&buffer);
    let sha1_hash: [u8; 20] = sha1_hasher.finalize().into();

    let mut sha256_hasher = Sha256::new();
    sha256_hasher.update(&buffer);
    let sha256_hash: [u8; 32] = sha256_hasher.finalize().into();

    let env = ExecutorEnv::builder()
        .write(&buffer)?
        .build()?;

    let prover = default_prover();
    let prove_info = prover.prove(env, HASHER_ELF)?;

    let proof = HashProof {
        receipt: prove_info.receipt,
        sha1_hash,
        sha256_hash,
    };

    let serialized = bincode::serialize(&proof)?;
    io::stdout().write_all(&serialized)?;

    Ok(())
}

fn verify() -> Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;
    
    let proof: HashProof = bincode::deserialize(&buffer)?;

    proof.receipt.verify(HASHER_ID)?;

    let journal = proof.receipt.journal.bytes.as_slice();
    
    let mut data_size_bytes = [0u8; 8];
    data_size_bytes.copy_from_slice(&journal[0..8]);
    let data_size = u64::from_le_bytes(data_size_bytes);
    
    let mut journal_sha1 = [0u8; 20];
    journal_sha1.copy_from_slice(&journal[8..28]);
    
    let mut journal_sha256 = [0u8; 32];
    journal_sha256.copy_from_slice(&journal[28..60]);

    eprintln!("Proof verified successfully!");
    eprintln!("Data size: {} bytes", data_size);
    eprintln!("SHA-1 hash: {}", hex::encode(journal_sha1));
    eprintln!("SHA-256 hash: {}", hex::encode(journal_sha256));
    eprintln!("Local SHA-1 hash: {}", hex::encode(proof.sha1_hash));
    eprintln!("Local SHA-256 hash: {}", hex::encode(proof.sha256_hash));

    if journal_sha1 == proof.sha1_hash && journal_sha256 == proof.sha256_hash {
        eprintln!("Hash verification successful!");
    } else {
        eprintln!("Hash verification failed!");
    }

    Ok(())
}
