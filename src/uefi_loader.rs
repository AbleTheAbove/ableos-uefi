use uefi::proto::console::gop::GraphicsOutput;
use uefi::{prelude::*, ResultExt};

use crate::{debug_kstate, info, kernel_state, kmain, KERNEL_STATE};

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

    //
    let year = system_table
        .runtime_services()
        .get_time()
        .unwrap()
        .unwrap()
        .year();
    info!("Year for sanity: {:?}", year);

    let a_bunch_of_handles = system_table
        .boot_services()
        .find_handles::<GraphicsOutput>()
        .unwrap()
        .unwrap();

    for handle in a_bunch_of_handles {
        info!["{:?}", handle];
        let handle_protocols = system_table
            .boot_services()
            .protocols_per_handle(handle)
            .unwrap()
            .unwrap();
        for protocol in handle_protocols.protocols() {
            info!["{:?}", protocol];
        }
    }

    // KMain should never return
    kmain(&system_table.runtime_services());
}
