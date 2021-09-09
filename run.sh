#!/bin/bash
export ABLEOS_LOADER="UEFI"
cargo build --release



uefi_run () {
   uefi-run -b assets/OVMF-pure-efi.fd target/x86_64-unknown-uefi/release/ableos.efi
}

# cargo install uefi-run
if [ $1 == 'u' ]
then
  export ABLEOS_LOADER="UEFI"
  uefi_run
elif [ $1 == 'r' ]
then
  # Place holder command
  export ABLEOS_LOADER="BIOS"
  echo $ABLEOS_LOADER
else
  export ABLEOS_LOADER="UEFI"
  uefi_run
fi
