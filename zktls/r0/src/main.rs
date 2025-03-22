use zktls_program_core::GuestInput;

fn main() {
    println!("Hello, world!");

    let input: GuestInput = risc0_zkvm::guest::env::read();

    println!("input: {:?}", input);

    let output = zktls_replayable_tls::entry_input(input);

    risc0_zkvm::guest::env::commit_slice(&output);
}
