#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![feature(alloc_error_handler)]
#![feature(default_alloc_error_handler)]
extern crate alloc;
extern crate rlibc;

pub mod able_graphics;
mod kernel_state;
mod uefi_loader;
use kernel_state::{debug_kstate, KERNEL_STATE};
pub use log::info;
pub mod util;

fn kmain() -> ! {
    loop {}
}
