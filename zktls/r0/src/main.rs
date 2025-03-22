use std::io::Read;

fn main() {
    let mut input = Vec::new();

    risc0_zkvm::guest::env::stdin()
        .read_to_end(&mut input)
        .unwrap();

    let output = zktls_replayable_tls::entry(&input);

    risc0_zkvm::guest::env::commit_slice(&output);
}
