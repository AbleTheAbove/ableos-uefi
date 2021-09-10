#!/bin/bash
cargo build --release
uefi-run -b assets/OVMF-pure-efi.fd target/x86_64-unknown-uefi/release/ableos.efi
