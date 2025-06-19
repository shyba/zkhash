#!/bin/bash

set -e

echo "Building Hash Prover..."

cargo build --release

echo "Build completed successfully!"

echo ""
echo "Creating test data (1024 bytes of zeros)..."
dd if=/dev/zero of=test_data.bin bs=2M count=1 2>/dev/null

echo "Testing proof generation and verification..."

echo "Generating proof..."
cat test_data.bin | ./target/release/host prove > test_proof.bin

echo "Verifying proof..."
if cat test_proof.bin | ./target/release/host verify > /dev/null 2>&1; then
    echo "✓ Test passed! Proof generation and verification working correctly."
else
    echo "✗ Test failed!"
    exit 1
fi

mv test_data.bin test_data_used.bin
mv test_proof.bin test_proof_used.bin

echo ""
echo "Usage:"
echo "  Generate proof: cat data.bin | ./target/release/host prove > proof.bin"
echo "  Verify proof:   cat proof.bin | ./target/release/host verify"
echo ""