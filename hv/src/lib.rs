//! `hv` is a high level safe Rust crate to access Hypervisor Framework.

use std::error;
use std::fmt;

/// Low level access to generated bindings.
pub use hv_sys as sys;
pub use vcpu::Vcpu;
pub use vm::Vm;

mod vcpu;
pub mod vm;

#[cfg(target_arch = "x86_64")]
pub mod x86;

/// Helper macro to call unsafe Hypervisor functions and map returned error codes to [Error] type.
#[macro_export]
macro_rules! call {
    ($f:expr) => {{
        let code = unsafe { $f };
        match code {
            ::hv_sys::HV_SUCCESS => Ok(()),
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
        match value {
            sys::HV_ERROR => Error::Unsuccessful,
            sys::HV_BUSY => Error::Busy,
            sys::HV_BAD_ARGUMENT => Error::BadArgument,
            sys::HV_NO_RESOURCES => Error::NoResources,
            sys::HV_NO_DEVICE => Error::NoDevice,
            sys::HV_UNSUPPORTED => Error::Unsupported,
            _ => Error::Unknown(value),
        }
    }
}
