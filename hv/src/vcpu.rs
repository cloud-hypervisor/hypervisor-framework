use std::ffi::c_void;
use std::mem;

use crate::{call, sys, Error, Space};

/// Represents a single virtual CPU.
///
/// [Vcpu] object is not thread safe, all calls must be performed from
/// the owning thread.
pub struct Vcpu(sys::hv_vcpuid_t);

impl Vcpu {
    /// Creates a vCPU instance for the current thread.
    pub(crate) fn new() -> Result<Vcpu, Error> {
        let mut handle: sys::hv_vcpuid_t = 0;
        call!(sys::hv_vcpu_create(&mut handle, 0))?;
        Ok(Vcpu(handle))
    }

    /// Associates the vCPU instance with an allocated address space.
    #[cfg(feature = "hv_10_15")]
    pub fn set_space(&self, asid: Space) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_space(self.0, asid))
    }

    /// Forces flushing of cached vCPU state.
    pub fn flush(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_flush(self.0))
    }

    /// Invalidates the TLB of a vCPU.
    pub fn invalidate_tlb(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_invalidate_tlb(self.0))
    }

    /// Executes a vCPU.
    pub fn run(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_run(self.0))
    }

    /// Executes a vCPU until the given deadline.
    #[cfg(feature = "hv_10_15")]
    pub fn run_until(&self, deadline: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_run_until(self.0, deadline))
    }

    /// Returns the cumulative execution time of a vCPU in nanoseconds.
    pub fn exec_time(&self) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vcpu_get_exec_time(self.0, &mut out))?;
        Ok(out)
    }

    /// Forces an immediate VMEXIT of the vCPU.
    pub fn interrupt(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_interrupt(mem::transmute(&self.0), 1))
    }

    /// Enables an MSR to be used natively by the VM.
    pub fn enable_native_msr(&self, msr: u32, enable: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_enable_native_msr(self.0, msr, enable))
    }

    /// Returns the current value of an MSR of a vCPU.
    pub fn read_msr(&self, msr: u32) -> Result<u64, Error> {
        let mut value = 0_u64;
        call!(sys::hv_vcpu_read_msr(self.0, msr, &mut value))?;
        Ok(value)
    }

    /// Set the value of an MSR of a vCPU.
    pub fn write_msr(&self, msr: u32, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_msr(self.0, msr, value))
    }
}

impl Drop for Vcpu {
    /// Destroys the vCPU instance associated with the current thread.
    fn drop(&mut self) {
        let _ = call!(sys::hv_vcpu_destroy(self.0));
    }
}

use crate::x86;

impl x86::VCpuX86Ext for Vcpu {
    /// Returns the current value of an architectural x86 register of a vCPU.
    fn read_register(&self, reg: x86::Reg) -> Result<u64, Error> {
        let mut value = 0_u64;
        call!(sys::hv_vcpu_read_register(
            self.0,
            reg as sys::hv_x86_reg_t,
            &mut value
        ))?;
        Ok(value)
    }

    /// Set the value of an architectural x86 register of a vCPU.
    fn write_register(&self, reg: x86::Reg, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_register(
            self.0,
            reg as sys::hv_x86_reg_t,
            value
        ))
    }

    /// Returns the current architectural x86 floating point and SIMD state of a vCPU.
    /// Structure and size are defined by the XSAVE feature set of the host processor.
    fn read_fpstate(&self, buffer: &mut [u8]) -> Result<(), Error> {
        call!(sys::hv_vcpu_read_fpstate(
            self.0,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as u64
        ))
    }

    /// Sets the architectural x86 floating point and SIMD state of a vCPU.
    fn write_fpstate(&self, buffer: &[u8]) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_fpstate(
            self.0,
            buffer.as_ptr() as *mut c_void,
            buffer.len() as u64
        ))
    }
}

use crate::vmx;

impl vmx::VCpuVmxExt for Vcpu {
    /// Returns the current value of a VMCS field of a vCPU.
    fn read_vmcs(&self, field: vmx::Vmcs) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vmx_vcpu_read_vmcs(self.0, field as u32, &mut out))?;
        Ok(out)
    }

    /// Set the value of a VMCS field of a vCPU.
    fn write_vmcs(&self, field: vmx::Vmcs, value: u64) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_write_vmcs(self.0, field as u32, value))
    }

    /// Returns the current value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn read_shadow_vmcs(&self, field: vmx::Vmcs) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vmx_vcpu_read_shadow_vmcs(
            self.0,
            field as u32,
            &mut out
        ))?;
        Ok(out)
    }

    /// Set the value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn write_shadow_vmcs(&self, field: vmx::Vmcs, value: u64) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_write_shadow_vmcs(
            self.0,
            field as u32,
            value
        ))
    }

    /// Set the access permissions of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn set_shadow_access(&self, field: vmx::Vmcs, flags: vmx::ShadowFlags) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_set_shadow_access(
            self.0,
            field as u32,
            flags.bits() as u64
        ))
    }
}
