//! `hv` is a high level safe Rust crate to access Hypervisor Framework.

use std::error;
use std::ffi::c_void;
use std::fmt;

/// Low level access to generated bindings.
pub use hv_sys as sys;
pub use vcpu::Vcpu;
pub use vm::Vm;

mod vcpu;
pub mod vm;

#[cfg(target_arch = "aarch64")]
pub mod arm64;
#[cfg(target_arch = "x86_64")]
pub mod x86;

pub type Size = u64;

/// Type of a user virtual address.
pub type Addr = *const c_void;

/// Type of a guest physical address.
pub type GPAddr = u64;

/// Helper macro to call unsafe Hypervisor functions and map returned error codes to [Error] type.
#[macro_export]
macro_rules! call {
    ($f:expr) => {{
        let code = unsafe { $f };
        match code {
            0 => Ok(()),
            _ => Err(Error::from(code)),
        }
    }};
}

/// The return type of framework functions.
/// Wraps the underlying `hv_return_t` type.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Error {
    Unsuccessful,
    Busy,
    BadArgument,
    NoResources,
    NoDevice,
    Unsupported,
    /// Not mapped error code.
    Unknown(sys::hv_return_t),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unsuccessful => write!(f, "The operation was unsuccessful"),
            Error::Busy => write!(f, "The operation was unsuccessful because the owning resource was busy"),
            Error::BadArgument => write!(f, "The operation was unsuccessful because the function call had an invalid argument"),
            Error::NoResources => write!(f, "The operation was unsuccessful because the host had no resources available to complete the request"),
            Error::NoDevice => write!(f, "The operation was unsuccessful because no VM or vCPU was available"),
            Error::Unsupported => write!(f, "The operation requested isn’t supported by the hypervisor"),
            Error::Unknown(code) => write!(f, "Error code: {}", *code as i32),
        }
    }
}

impl From<sys::hv_return_t> for Error {
    fn from(value: sys::hv_return_t) -> Self {
        // Looks like bindgen gets confused sometimes and produces different code for these
        // constants (`sys::HV_ERROR` vs `hv_return_t_HV_ERROR`) on different machines making things
        // to fail. It's probably easier to just hardcode them.
        let v = value as i64;
        match v {
            0xfae94001 => Error::Unsuccessful,
            0xfae94002 => Error::Busy,
            0xfae94003 => Error::BadArgument,
            0xfae94005 => Error::NoResources,
            0xfae94006 => Error::NoDevice,
            0xfae9400f => Error::Unsupported,
            _ => Error::Unknown(value),
        }
    }
}
