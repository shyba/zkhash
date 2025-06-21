# zkHash: Zero-Knowledge Hash Verification

An experiment consisting of a collection of zero-knowledge systems for proving and verifying SHA-1 and SHA-256 hash computations. For now,  using both RISC Zero and SP1 proving systems.

One possible application could be for someone with a SHA-1 hash to look up for an equivalent SHA-256 without having the data at hand.

**DISCLAIMER: DO NOT USE IN PRODUCTION** (this is just a toy project without any audit)

## Implementations

### RISC Zero Implementation (`risc0-sha2+sha1/`)

A RISC Zero zkVM program that computes SHA-1 and SHA-256 hashes with proof generation and verification.

**Architecture:**
- **Host** (`host/`): Rust application that generates and verifies proofs
- **Guest** (`methods/guest/`): zkVM program that computes hashes inside the zero-knowledge environment
- **Methods** (`methods/`): Build configuration and generated zkVM bytecode

**Requirements:**
- Rust (latest stable)
- RISC Zero toolchain

### SP1 Implementation (`sp1-sha2+sha1/`)

An SP1 program that computes SHA-1 and SHA-256 hashes with support for core proofs and EVM-compatible proofs.

**Architecture:**
- **Program** (`program/`): SP1 program that performs hash computations
- **Script** (`script/`): Proof generation and verification scripts
- **Lib** (`lib/`): Shared library code

**Requirements:**
- Rust (latest stable)
- SP1 toolchain

## Quick Start

### RISC Zero
```bash
cd risc0-sha2+sha1
./build.sh

# Generate and verify proof
cat data.bin | ./target/release/host prove > proof.bin
cat proof.bin | ./target/release/host verify
```

### SP1
```bash
cd sp1-sha2+sha1/script

# Execute program
cargo run --release -- --execute

# Generate SP1 core proof
cargo run --release -- --prove

# Generate EVM-compatible proof (requires 16GB+ RAM)
cargo run --release --bin evm -- --system groth16
```

## How it Works

1. **Proof Generation**: The host/script reads input data and sends it to the zkVM program
2. **zkVM Execution**: The program commits the data size and computes SHA-1 and SHA-256 hashes
3. **Proof Creation**: The zkVM generates a cryptographic proof that the hash computations were performed correctly
4. **Verification**: The verifier checks proof validity and compares zkVM-computed hashes with locally computed hashes

## Security Properties

- **Zero-Knowledge**: The input data remains private; only the hash outputs are revealed
- **Soundness**: Invalid computations cannot produce valid proofs
- **Completeness**: Valid computations always produce verifiable proofs

## License

Licensed under the Apache License, Version 2.0.
