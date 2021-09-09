#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate rlibc;

mod kernel_state;
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
