#!/bin/bash

uefi_run () {
   cargo build --release &&
   uefi-run -b assets/OVMF-pure-efi.fd target/x86_64-unknown-uefi/release/ableos.efi
}

# cargo install uefi-run
if [ $1 == 'u' ]
then
  uefi_run
elif [ $1 == 'r' ]
then
  # Place holder command
  echo "Oops"
else
  uefi_run
fi
