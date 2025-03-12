#!/bin/bash

# TODO: Add check for `cargo` `cargo prove`, `cargo risczero` and `docker`

RISC0_TLS_ELF_PATH=target/riscv-guest/riscv32im-risc0-zkvm-elf/docker/zktls_r0/zktls-r0
SP1_TLS_ELF_PATH=zktls/sp1/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/zktls-sp1

TARGET_ELF_OUTPUTS_PATH=./target/elf-outputs
SP1_TLS_ELF_FILENAME=zktls-sp1
R0_TLS_ELF_FILENAME=zktls-r0

cargo build --release || exit 1

echo "Building zktls-sp1"
cd zktls/sp1 || exit 1
cargo prove build --docker --workspace-directory ../../ || exit 1
cd ../../ || exit 1

echo "Building zktls-r0"
cargo risczero build --manifest-path zktls/r0/Cargo.toml || exit 1

echo "Creating elf outputs"

mkdir -p $TARGET_ELF_OUTPUTS_PATH || exit 1

cp $SP1_TLS_ELF_PATH $TARGET_ELF_OUTPUTS_PATH/$SP1_TLS_ELF_FILENAME || exit 1
cp $RISC0_TLS_ELF_PATH $TARGET_ELF_OUTPUTS_PATH/$R0_TLS_ELF_FILENAME || exit 1

echo "Generating image id and vkey"

target/release/compute-imageid $TARGET_ELF_OUTPUTS_PATH/$R0_TLS_ELF_FILENAME > $TARGET_ELF_OUTPUTS_PATH/$R0_TLS_ELF_FILENAME.key || exit 1
target/release/compute-vkey $TARGET_ELF_OUTPUTS_PATH/$SP1_TLS_ELF_FILENAME > $TARGET_ELF_OUTPUTS_PATH/$SP1_TLS_ELF_FILENAME.vkey || exit 1
