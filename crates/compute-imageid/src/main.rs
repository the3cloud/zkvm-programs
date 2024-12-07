use std::{env, fs};

fn main() {
    let mut args = env::args();

    let _ = args.next().unwrap();
    let path = args.next().unwrap();

    let bytes = fs::read(path).unwrap();

    let imageid = risc0_zkvm::compute_image_id(&bytes).unwrap();

    println!("0x{}", imageid);
}
