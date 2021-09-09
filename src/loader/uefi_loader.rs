use uefi::{prelude::*, ResultExt};

use crate::debug_kstate;
use crate::info;
use crate::kernel_state;
use crate::kmain;
use crate::KERNEL_STATE;

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