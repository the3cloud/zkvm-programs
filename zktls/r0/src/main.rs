use zktls_program_core::GuestInput;

use std::io::Read;

fn main() {
    println!("Hello, world!");
    let mut input = Vec::new();

    let length = risc0_zkvm::guest::env::stdin()
        .read_to_end(&mut input)
        .unwrap();

    println!("length: {}", length);

    let output = zktls_replayable_tls::entry(&input);

    risc0_zkvm::guest::env::commit_slice(&output);
}
