use std::ffi::c_void;
use std::sync::Arc;

use crate::{call, sys, Addr, Error, GPAddr, Memory, Size, Vcpu};

#[cfg(target_arch = "x86_64")]
pub type Options = crate::x86::VmOptions;

#[cfg(target_arch = "aarch64")]
pub type Options = sys::hv_vm_config_t;

/// Vm is an entry point to Hypervisor Framework.
#[derive(Debug)]
pub struct Vm;

/// Destroys the VM instance associated with the current process.
impl Drop for Vm {
    fn drop(&mut self) {
        call!(sys::hv_vm_destroy()).unwrap()
    }
}

unsafe impl Send for Vm {}

impl Vm {
    /// Creates a VM instance for the current process.
    ///
    /// Only one VM object can exists at a time.
    /// All subsequent attempts will return an error from Hypervisor Framework.
    ///
    /// In order to create child objects (`Vcpu`, `Space`, etc), this object must be wrapped
    /// with [Arc].
    ///
    pub fn new(options: Options) -> Result<Vm, Error> {
        #[cfg(target_arch = "x86_64")]
        let options = options.bits();

        call!(sys::hv_vm_create(options))?;
        Ok(Vm)
    }

    /// Creates a vCPU instance for the current thread.
    ///
    /// `create_cpu` implements safe wrapper around `hv_vcpu_create` that holds reference to the
    /// [Vm] object, so they can be dropped in proper order.
    pub fn create_cpu(self: Arc<Self>) -> Result<Vcpu, Error> {
        Vcpu::new(Arc::clone(&self))
    }

    /// Maps a region in the virtual address space of the current task into the guest physical
    /// address space of the VM.
    ///
    /// The host memory must encompass a single VM region, typically allocated with `mmap` or
    /// `mach_vm_allocate` instead of `malloc`. [1]
    ///
    /// # Arguments
    /// * `uva` - Page aligned virtual address in the current task.
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be mapped.
    /// * `flags` - READ, WRITE and EXECUTE permissions of the region
    ///
    /// [1]: https://developer.apple.com/documentation/hypervisor/1441187-hv_vm_map
    ///
    pub fn map(&self, uva: Addr, gpa: GPAddr, size: Size, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_map(
            uva as *mut c_void,
            gpa,
            size,
            flags.bits() as _
        ))
    }

    /// Unmaps a region in the guest physical address space of the VM
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be unmapped.
    pub fn unmap(&self, gpa: GPAddr, size: Size) -> Result<(), Error> {
        call!(sys::hv_vm_unmap(gpa, size))
    }

    /// Modifies the permissions of a region in the guest physical address space of the VM.
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be modified.
    /// * `flags` - New READ, WRITE and EXECUTE permissions of the region.
    pub fn protect(&self, gpa: GPAddr, size: Size, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_protect(gpa, size, flags.bits() as _))
    }
}
