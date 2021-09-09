#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;
extern crate rlibc;

use log::info;
use uefi::prelude::*;
use uefi::ResultExt;

use crate::alloc::vec;
use crate::kernel_state::debug_kstate;

use crate::kernel_state::KERNEL_STATE;

pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";
// Multimedia shell
mod kernel_state;
#[entry]
fn pre_main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    // Print out the UEFI revision number
    let _return = system_table.stdout().clear();
    // add UEFI version to kstate
    {
        let rev = system_table.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());
        let uefi_info = kernel_state::UEFI {
            major: major,
            minor: minor,
        };
        KERNEL_STATE.lock().loader.uefi = Some(uefi_info);
    }
    debug_kstate();

    //
    let x = system_table.runtime_services().get_time();
    info!("{:?}", x.unwrap().unwrap().year());

    // KMain should never return
    kmain();
}

fn kmain() -> ! {
    test_alloc();
    loop {}
}
fn test_alloc() {
    let _x = vec!["hi"];
}
