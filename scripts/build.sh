#!/bin/bash

# TODO: Add check for `cargo prove`, `cargo risczero` and `docker`

echo "Building zktls-sp1"
cargo prove build --docker --binary zktls-sp1 || exit 1

echo "Building zktls-r0"
cargo risczero build --manifest-path zktls/r0/Cargo.toml || exit 1

cargo prove vkey --elf target/elf-compilation/docker/riscv32im-succinct-zkvm-elf/release/zktls-sp1 || exit 1