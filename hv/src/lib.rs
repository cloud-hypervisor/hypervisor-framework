//! `hv` is a high level safe Rust crate to access Hypervisor Framework.

use std::error;
use std::fmt;

/// Low level access to generated bindings.
pub use hv_sys as sys;

mod vcpu;
pub mod vmx;
pub mod x86;

pub use vcpu::Vcpu;

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

/// The type of system capabilities.
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Capability {
    VCpuMax = sys::HV_CAP_VCPUMAX,
    AddrSpaceMax = sys::HV_CAP_ADDRSPACEMAX,
}

/// Type of a user virtual address.
pub type UVAddr = sys::hv_uvaddr_t;

/// Type of a guest physical address.
pub type GPAddr = sys::hv_gpaddr_t;

/// Type of a guest address space.
pub type Space = sys::hv_vm_space_t;

pub const VM_SPACE_DEFAULT: Space = sys::HV_VM_SPACE_DEFAULT;

/// Guest physical memory region permissions.
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Memory {
    Read = sys::HV_MEMORY_READ,
    Write = sys::HV_MEMORY_WRITE,
    Exec = sys::HV_MEMORY_EXEC,
}

/// Creates a VM instance for the current process.
pub fn vm_create() -> Result<(), Error> {
    call!(sys::hv_vm_create(0))
}

/// Gets the value of capabilities of the system.
pub fn capability(cap: Capability) -> Result<u64, Error> {
    let mut out = 0_u64;
    call!(sys::hv_capability(cap as sys::hv_capability_t, &mut out))?;
    Ok(out)
}

/// Creates an additional guest address space for the current task.
#[cfg(feature = "hv_10_15")]
pub fn vm_space_create() -> Result<Space, Error> {
    let mut space: Space = 0;
    call!(sys::hv_vm_space_create(&mut space))?;
    Ok(space)
}

/// Destroys the address space instance associated with the current task.
///
/// # Arguments
/// * `asid` - Address space ID
#[cfg(feature = "hv_10_15")]
pub fn vm_space_destroy(asid: Space) -> Result<(), Error> {
    call!(sys::hv_vm_space_destroy(asid))
}

/// Maps a region in the virtual address space of the current task into the guest physical
/// address space of the VM.
///
/// # Arguments
/// * `uva` - Page aligned virtual address in the current task.
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be mapped.
/// * `flags` - READ, WRITE and EXECUTE permissions of the region
pub fn vm_map(uva: UVAddr, gpa: GPAddr, size: u64, flags: Memory) -> Result<(), Error> {
    call!(sys::hv_vm_map(
        uva,
        gpa,
        size,
        flags as sys::hv_memory_flags_t
    ))
}

/// Unmaps a region in the guest physical address space of the VM
///
/// # Arguments
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be unmapped.
pub fn vm_unmap(gpa: GPAddr, size: u64) -> Result<(), Error> {
    call!(sys::hv_vm_unmap(gpa, size))
}

/// Modifies the permissions of a region in the guest physical address space of the VM.
///
/// # Arguments
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be modified.
/// * `flags` - New READ, WRITE and EXECUTE permissions of the region.
pub fn vm_protect(gpa: GPAddr, size: u64, flags: Memory) -> Result<(), Error> {
    call!(sys::hv_vm_protect(
        gpa,
        size,
        flags as sys::hv_memory_flags_t
    ))
}

/// Maps a region in the virtual address space of the current task
/// into a guest physical address space of the VM.
///
/// # Arguments
/// * `asid` - Address space ID.
/// * `uva` - Page aligned virtual address in the current task.
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be mapped.
/// * `flags` - READ, WRITE and EXECUTE permissions of the region.
#[cfg(feature = "hv_10_15")]
pub fn vm_map_space(
    asid: Space,
    uva: UVAddr,
    gpa: GPAddr,
    size: u64,
    flags: Memory,
) -> Result<(), Error> {
    call!(sys::hv_vm_map_space(
        asid,
        uva,
        gpa,
        size,
        flags as sys::hv_memory_flags_t
    ))
}

/// Unmaps a region in a guest physical address space of the VM.
///
/// # Arguments
/// * `asid` - Address space ID.
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be unmapped.
#[cfg(feature = "hv_10_15")]
pub fn vm_unmap_space(asid: Space, gpa: GPAddr, size: u64) -> Result<(), Error> {
    call!(sys::hv_vm_unmap_space(asid, gpa, size))
}

/// Modifies the permissions of a region in a guest physical address space of the VM.
///
/// # Arguments
/// * `asid` - Address space ID.
/// * `gpa` - Page aligned address in the guest physical address space.
/// * `size` - Size in bytes of the region to be modified.
/// * `flags` - New READ, WRITE and EXECUTE permissions of the region.
#[cfg(feature = "hv_10_15")]
pub fn vm_protect_space(asid: Space, gpa: GPAddr, size: u64, flags: Memory) -> Result<(), Error> {
    call!(sys::hv_vm_protect_space(
        asid,
        gpa,
        size,
        flags as sys::hv_memory_flags_t
    ))
}

/// Synchronizes guest TSC across all vCPUs.
pub fn vm_sync_tsc(tcs: u64) -> Result<(), Error> {
    call!(sys::hv_vm_sync_tsc(tcs))
}

/// Destroys the VM instance associated with the current process.
pub fn vm_destroy() -> Result<(), Error> {
    call!(sys::hv_vm_destroy())
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
