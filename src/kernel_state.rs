use crate::info;
use alloc::string::String;
use alloc::string::ToString;
use core::fmt;
use lazy_static::lazy_static;
/// TODO: owo what is this?
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
#[cfg(debug_assertions)]
/// A constant to check if the kernel is in debug mode
pub const RELEASE_TYPE: &str = "debug";
#[cfg(not(debug_assertions))]
/// A constant to check if the kernel is in release mode
pub const RELEASE_TYPE: &str = "release";

lazy_static! {
    pub static ref KERNEL_STATE: spin::Mutex<KernelState> = {
        let state = KernelState {
            version: KernelVersion {
                version_str: KERNEL_VERSION.to_string(),
                release_type: RELEASE_TYPE.to_string(),
            },
            serial_log: true,
            loader: Loader { uefi: None },
        };
        spin::Mutex::new(state)
    };
}

/// KernelState
#[derive(Debug)]
pub struct KernelState {
    /// The first value is the release state and the second is the version string
    pub version: KernelVersion,
    /// This declares whether debug should be logged
    pub serial_log: bool,

    pub loader: Loader,
}
impl fmt::Display for KernelState {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}\n{}", self.version, self.serial_log)
    }
}

/// Kernel Versioning used to assist in debugging
#[derive(Debug)]
pub struct KernelVersion {
    /// A semantic versioning
    pub version_str: String,
    /// The release type of the kernel "release" or "debug"
    pub release_type: String,
}
impl fmt::Display for KernelVersion {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{} {}", self.version_str, self.release_type)
    }
}

#[derive(Debug)]
pub struct Loader {
    pub uefi: Option<UEFI>,
}

#[derive(Debug)]
pub struct UEFI {
    pub major: u16,
    pub minor: u16,
}

pub fn debug_kstate() {
    info!("Kernel Version: {}", KERNEL_STATE.lock().version);
    info!("Kernel Logging: {:?}", KERNEL_STATE.lock().serial_log);
}
