use uefi::{prelude::*, ResultExt};

use crate::able_graphics::{draw_fb, fill_color, set_graphics_mode};

use crate::{debug_kstate, info, kernel_state, kmain, KERNEL_STATE};
use log::warn;
use uefi::proto::console::gop::GraphicsOutput;

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

    /*
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
    */
    {
        info!("Running graphics output protocol test");
        if let Ok(gop) = system_table
            .boot_services()
            .locate_protocol::<GraphicsOutput>()
        {
            let gop = gop.expect("Warnings encountered while opening GOP");
            // Maybe save this
            let gop = unsafe { &mut *gop.get() };

            set_graphics_mode(gop);
            fill_color(gop);
            draw_fb(gop);

            //crate::check_screenshot(bt, "gop_test");
        } else {
            // No tests can be run.
            warn!("UEFI Graphics Output Protocol is not supported");
        }
    }
    // KMain should never return
    kmain();
}
