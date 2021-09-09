#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]
extern crate alloc;
extern crate rlibc;

mod kernel_state;
//mod loader;
// If using uefi then loader/uefi_loader
// else loader/bios.rs

#[cfg_attr(feature = "uefi", path = "loader/uefi_loader.rs")]
#[cfg_attr(feature = "bios", path = "loader/bio.rs")]
mod loader;

use crate::{
    alloc::vec,
    kernel_state::{debug_kstate, KERNEL_STATE},
};
use log::info;

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";
// Multimedia shell

fn kmain() -> ! {
    test_alloc();

    loop {}
}
fn test_alloc() {
    let _x = vec!["hi"];
}
