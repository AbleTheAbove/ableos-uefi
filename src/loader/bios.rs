#[export_name = "_start"]
pub extern "C" fn __impl_start(_boot_info: &'static ::bootloader::bootinfo::BootInfo) -> ! {
    loop {}
}
