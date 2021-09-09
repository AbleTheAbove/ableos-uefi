 #uefi-run -b OVMF-pure-efi.fd -q /path/to/qemu app.efi # -- <extra_qemu_args>

 # cargo install uefi-run
cargo build --release &&
uefi-run -b assets/OVMF-pure-efi.fd target/x86_64-unknown-uefi/release/ableos.efi
