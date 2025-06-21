//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use hash_lib::PublicValuesStruct;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const HASH_ELF: &[u8] = include_elf!("hash-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    execute: bool,

    #[arg(long)]
    prove: bool,

    #[arg(long, default_value = "Hello, World!")]
    data: String,
}

fn main() {
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    let mut args = Args::parse();

    args.data = String::from_utf8(vec![b'X'; 8192]).unwrap();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let client = ProverClient::from_env();

    let mut stdin = SP1Stdin::new();
    let data_bytes = args.data.as_bytes().to_vec();
    stdin.write(&data_bytes);

    println!("data: {}", &args.data[..80]);

    if args.execute {
        let (output, report) = client.execute(HASH_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        let decoded = PublicValuesStruct::abi_decode(output.as_slice()).unwrap();
        let PublicValuesStruct { size, sha1, sha2 } = decoded;
        println!("size: {}", size);
        println!("sha1: {}", hex::encode(sha1));
        println!("sha2: {}", hex::encode(sha2));

        let (expected_sha1, expected_sha256) = hash_lib::hash(data_bytes);
        assert_eq!(sha1, expected_sha1);
        assert_eq!(sha2, expected_sha256);
        println!("Hash values are correct!");

        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        let (pk, vk) = client.setup(HASH_ELF);

        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
