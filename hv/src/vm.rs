use crate::{call, sys, Error, Vcpu};

bitflags::bitflags! {
    /// Guest physical memory region permissions.
    pub struct Memory: u32 {
        const READ = sys::HV_MEMORY_READ;
        const WRITE = sys::HV_MEMORY_WRITE;
        const EXEC = sys::HV_MEMORY_EXEC;
    }
}

/// Vm is an entry point to Hypervisor Framework.
#[derive(Debug)]
pub struct Vm;

impl Vm {
    /// Creates a vCPU instance for the current thread.
    pub fn create_cpu() -> Result<Vcpu, Error> {
        Vcpu::new()
    }

    /// Destroys the VM instance associated with the current process.
    pub fn destroy() -> Result<(), Error> {
        call!(sys::hv_vm_destroy())
    }
}
