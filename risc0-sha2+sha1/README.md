# RISC Zero zkHash

A demo RISC0 ZKVM program for verifying SHA-1 and SHA-256 hash computations.

## Overview

This project demonstrates how to compute cryptographic hashes (SHA-1 and SHA-256) inside a zero-knowledge virtual machine and generate proofs that the computations were performed correctly.

One possible application could be for someone with a SHA-1 hash to look up for an equivalent SHA-256 without having the data at hand.

DISCLAIMER: DO NOT USE IN PRODUCTION (this is just a toy project without any audit)

## Architecture

- **Host** (`host/`): Rust application that generates and verifies proofs
- **Guest** (`methods/guest/`): zkVM program that computes hashes inside the zero-knowledge environment
- **Methods** (`methods/`): Build configuration and generated zkVM bytecode

## Requirements

- Rust (latest stable)
- RISC Zero toolchain

## Usage

### Build the project
```bash
./build.sh
```

### Generate a proof
```bash
cat data.bin | ./target/release/host prove > proof.bin
```

### Verify a proof
```bash
cat proof.bin | ./target/release/host verify
```

## Input Requirements

- Input data can be of any size
- Data is read from stdin for proof generation  
- Proof is output as binary data to stdout
- Data size is committed to the proof for verification

## How it works

1. **Proof Generation**: The host reads input data of any size and sends it to the zkVM guest program
2. **zkVM Execution**: The guest commits the data size and computes SHA-1 and SHA-256 hashes of the input data
3. **Proof Creation**: The zkVM generates a cryptographic proof that the hash computations were performed correctly
4. **Verification**: The verifier checks the proof validity, extracts the data size, and compares the zkVM-computed hashes with locally computed hashes

## Security Properties
- **Zero-Knowledge**: The input data remains private; only the hash outputs are revealed
- **Soundness**: Invalid computations cannot produce valid proofs
- **Completeness**: Valid computations always produce verifiable proofs

## Example

```bash
# Create test data (any size)
dd if=/dev/zero of=test.bin bs=1024 count=1

# Generate proof
cat test.bin | ./target/release/host prove > proof.bin

# Verify proof
cat proof.bin | ./target/release/host verify
```

## License

Licensed under the Apache License, Version 2.0.