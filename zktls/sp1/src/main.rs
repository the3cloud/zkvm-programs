fn main() {
    println!("cycle-tracker-start: tls");
    let input = sp1_zkvm::io::read_vec();

    let output = zktls_replayable_tls::entry(&input);

    sp1_zkvm::io::commit_slice(&output);
    println!("cycle-tracker-end: tls");
}
