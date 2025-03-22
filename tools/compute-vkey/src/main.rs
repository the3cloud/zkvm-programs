use std::{env, fs};

use sp1_sdk::{HashableKey, ProverClient};

fn main() {
    let mut args = env::args();

    let _ = args.next().unwrap();
    let path = args.next().unwrap();

    let bytes = fs::read(path).unwrap();

    std::env::set_var("SP1_PROVER", "mock");
    let client = ProverClient::from_env();

    let (_, vkey) = client.setup(&bytes);

    println!("{}", vkey.bytes32());
}
