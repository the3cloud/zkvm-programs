use zktls_program_core::GuestInput;

fn main() {
    let input: GuestInput = risc0_zkvm::guest::env::read();

    let output = zktls_replayable_tls::entry_input(input);

    risc0_zkvm::guest::env::commit(&output);
}
