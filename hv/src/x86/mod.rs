//! x86 specific routines.

use std::ffi::c_void;
use std::mem;
use std::sync::Arc;

use crate::{call, sys, Addr, Error, GPAddr, Memory, Size, Vcpu, Vm};

pub mod vmx;

pub type UVAddr = Addr;

/// Type of a guest address space.
pub type SpaceId = sys::hv_vm_space_t;

pub const VM_SPACE_DEFAULT: SpaceId = sys::HV_VM_SPACE_DEFAULT;

/// The type of system capabilities.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Capability {
    VcpuMax = 0,
    AddrSpaceMax = 1,
}

bitflags::bitflags! {
    pub struct VmOptions: u64 {
        const DEFAULT = sys::HV_VM_DEFAULT as _;
        const SPECIFY_MITIGATIONS = sys::HV_VM_SPECIFY_MITIGATIONS as _;
        const MITIGATION_A_ENABLE = sys::HV_VM_MITIGATION_A_ENABLE as _;
        const MITIGATION_B_ENABLE = sys::HV_VM_MITIGATION_B_ENABLE as _;
        const MITIGATION_C_ENABLE = sys::HV_VM_MITIGATION_C_ENABLE as _;
        const MITIGATION_D_ENABLE = sys::HV_VM_MITIGATION_D_ENABLE as _;
        const MITIGATION_E_ENABLE = sys::HV_VM_MITIGATION_E_ENABLE as _;
    }
}

impl Default for VmOptions {
    fn default() -> Self {
        VmOptions::DEFAULT
    }
}

/// Represents an additional guest address space.
#[cfg(feature = "hv_10_15")]
#[derive(Debug)]
pub struct Space {
    vm: Arc<Vm>,
    id: SpaceId,
}

#[cfg(feature = "hv_10_15")]
impl Space {
    fn new(vm: Arc<Vm>) -> Result<Space, Error> {
        let mut id: SpaceId = 0;
        call!(sys::hv_vm_space_create(&mut id))?;
        Ok(Space { vm, id })
    }

    /// Returns the underlying space id.
    #[inline]
    pub fn id(&self) -> SpaceId {
        self.id
    }

    /// Maps a region in the virtual address space of the current task
    /// into a guest physical address space of the VM.
    ///
    /// # Arguments
    /// * `uva` - Page aligned virtual address in the current task.
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be mapped.
    /// * `flags` - READ, WRITE and EXECUTE permissions of the region.
    pub fn map(&self, uva: UVAddr, gpa: GPAddr, size: u64, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_map_space(
            self.id,
            uva as *const c_void,
            gpa,
            size,
            flags.bits() as _
        ))
    }

    /// Unmaps a region in a guest physical address space of the VM.
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be unmapped.
    pub fn unmap(&self, gpa: GPAddr, size: Size) -> Result<(), Error> {
        call!(sys::hv_vm_unmap_space(self.id, gpa, size))
    }

    /// Modifies the permissions of a region in a guest physical address space of the VM.
    ///
    /// # Arguments
    /// * `gpa` - Page aligned address in the guest physical address space.
    /// * `size` - Size in bytes of the region to be modified.
    /// * `flags` - New READ, WRITE and EXECUTE permissions of the region.
    pub fn protect(&self, gpa: GPAddr, size: Size, flags: Memory) -> Result<(), Error> {
        call!(sys::hv_vm_protect_space(
            self.id,
            gpa,
            size,
            flags.bits() as _
        ))
    }
}

#[cfg(feature = "hv_10_15")]
impl Drop for Space {
    fn drop(&mut self) {
        call!(sys::hv_vm_space_destroy(self.id)).unwrap()
    }
}

pub trait VmExt {
    /// Gets the value of capabilities of the system.
    fn capability(&self, cap: Capability) -> Result<u64, Error>;

    /// Creates an additional guest address space for the current task.
    #[cfg(feature = "hv_10_15")]
    fn create_space(self: Arc<Self>) -> Result<Space, Error>;

    /// Synchronizes guest TSC across all vCPUs.
    fn sync_tsc(tcs: u64) -> Result<(), Error>;
}

/// x86 specific routines for vCPU.
pub trait VcpuExt {
    /// Executes a vCPU until the given deadline.
    #[cfg(feature = "hv_10_15")]
    fn run_until(&self, deadline: u64) -> Result<(), Error>;

    /// Forces flushing of cached vCPU state.
    fn flush(&self) -> Result<(), Error>;

    /// Invalidates the TLB of a vCPU.
    fn invalidate_tlb(&self) -> Result<(), Error>;

    /// Associates the vCPU instance with an allocated address space.
    #[cfg(feature = "hv_10_15")]
    fn set_space(&self, space: &Space) -> Result<(), Error>;

    /// Forces an immediate VMEXIT of the vCPU.
    fn interrupt(&self) -> Result<(), Error>;

    /// Enables an MSR to be used natively by the VM.
    fn enable_native_msr(&self, msr: u32, enable: bool) -> Result<(), Error>;

    /// Returns the current value of an MSR of a vCPU.
    fn read_msr(&self, msr: u32) -> Result<u64, Error>;

    /// Set the value of an MSR of a vCPU.
    fn write_msr(&self, msr: u32, value: u64) -> Result<(), Error>;

    /// Returns the current value of an architectural x86 register of a vCPU.
    fn read_register(&self, reg: Reg) -> Result<u64, Error>;

    /// Set the value of an architectural x86 register of a vCPU.
    fn write_register(&self, reg: Reg, value: u64) -> Result<(), Error>;

    /// Returns the current architectural x86 floating point and SIMD state of a vCPU.
    /// Structure and size are defined by the XSAVE feature set of the host processor.
    fn read_fpstate(&self, buffer: &mut [u8]) -> Result<(), Error>;

    /// Sets the architectural x86 floating point and SIMD state of a vCPU.
    fn write_fpstate(&self, buffer: &[u8]) -> Result<(), Error>;
}

impl VmExt for Vm {
    /// Gets the value of capabilities of the system.
    fn capability(&self, cap: Capability) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_capability(cap as u64, &mut out))?;
        Ok(out)
    }

    /// Creates an additional guest address space for the current task.
    #[cfg(feature = "hv_10_15")]
    fn create_space(self: Arc<Self>) -> Result<Space, Error> {
        Space::new(Arc::clone(&self))
    }

    /// Synchronizes guest TSC across all vCPUs.
    fn sync_tsc(tcs: u64) -> Result<(), Error> {
        call!(sys::hv_vm_sync_tsc(tcs))
    }
}

impl VcpuExt for Vcpu {
    /// Executes a vCPU until the given deadline.
    #[cfg(feature = "hv_10_15")]
    fn run_until(&self, deadline: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_run_until(self.id, deadline))
    }

    /// Forces flushing of cached vCPU state.
    fn flush(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_flush(self.id))
    }

    /// Invalidates the TLB of a vCPU.
    fn invalidate_tlb(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_invalidate_tlb(self.id))
    }

    /// Associates the vCPU instance with an allocated address space.
    #[cfg(feature = "hv_10_15")]
    fn set_space(&self, space: &Space) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_space(self.id, space.id()))
    }

    /// Forces an immediate VMEXIT of the vCPU.
    fn interrupt(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_interrupt(mem::transmute(&self.id), 1))
    }

    /// Enables an MSR to be used natively by the VM.
    fn enable_native_msr(&self, msr: u32, enable: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_enable_native_msr(self.id, msr, enable))
    }

    /// Returns the current value of an MSR of a vCPU.
    fn read_msr(&self, msr: u32) -> Result<u64, Error> {
        let mut value = 0_u64;
        call!(sys::hv_vcpu_read_msr(self.id, msr, &mut value))?;
        Ok(value)
    }

    /// Set the value of an MSR of a vCPU.
    fn write_msr(&self, msr: u32, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_msr(self.id, msr, value))
    }

    /// Returns the current value of an architectural x86 register of a vCPU.
    fn read_register(&self, reg: Reg) -> Result<u64, Error> {
        let mut value = 0_u64;
        call!(sys::hv_vcpu_read_register(
            self.id,
            reg as sys::hv_x86_reg_t,
            &mut value
        ))?;
        Ok(value)
    }

    /// Set the value of an architectural x86 register of a vCPU.
    fn write_register(&self, reg: Reg, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_register(
            self.id,
            reg as sys::hv_x86_reg_t,
            value
        ))
    }

    /// Returns the current architectural x86 floating point and SIMD state of a vCPU.
    /// Structure and size are defined by the XSAVE feature set of the host processor.
    fn read_fpstate(&self, buffer: &mut [u8]) -> Result<(), Error> {
        call!(sys::hv_vcpu_read_fpstate(
            self.id,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as u64
        ))
    }

    /// Sets the architectural x86 floating point and SIMD state of a vCPU.
    fn write_fpstate(&self, buffer: &[u8]) -> Result<(), Error> {
        call!(sys::hv_vcpu_write_fpstate(
            self.id,
            buffer.as_ptr() as *mut c_void,
            buffer.len() as u64
        ))
    }
}

/// x86 architecture register IDs.
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reg {
    RIP = sys::hv_x86_reg_t_HV_X86_RIP,
    RFLAGS = sys::hv_x86_reg_t_HV_X86_RFLAGS,
    RAX = sys::hv_x86_reg_t_HV_X86_RAX,
    RCX = sys::hv_x86_reg_t_HV_X86_RCX,
    RDX = sys::hv_x86_reg_t_HV_X86_RDX,
    RBX = sys::hv_x86_reg_t_HV_X86_RBX,
    RSI = sys::hv_x86_reg_t_HV_X86_RSI,
    RDI = sys::hv_x86_reg_t_HV_X86_RDI,
    RSP = sys::hv_x86_reg_t_HV_X86_RSP,
    RBP = sys::hv_x86_reg_t_HV_X86_RBP,
    R8 = sys::hv_x86_reg_t_HV_X86_R8,
    R9 = sys::hv_x86_reg_t_HV_X86_R9,
    R10 = sys::hv_x86_reg_t_HV_X86_R10,
    R11 = sys::hv_x86_reg_t_HV_X86_R11,
    R12 = sys::hv_x86_reg_t_HV_X86_R12,
    R13 = sys::hv_x86_reg_t_HV_X86_R13,
    R14 = sys::hv_x86_reg_t_HV_X86_R14,
    R15 = sys::hv_x86_reg_t_HV_X86_R15,
    CS = sys::hv_x86_reg_t_HV_X86_CS,
    SS = sys::hv_x86_reg_t_HV_X86_SS,
    DS = sys::hv_x86_reg_t_HV_X86_DS,
    ES = sys::hv_x86_reg_t_HV_X86_ES,
    FS = sys::hv_x86_reg_t_HV_X86_FS,
    GS = sys::hv_x86_reg_t_HV_X86_GS,
    IDT_BASE = sys::hv_x86_reg_t_HV_X86_IDT_BASE,
    IDT_LIMIT = sys::hv_x86_reg_t_HV_X86_IDT_LIMIT,
    GDT_BASE = sys::hv_x86_reg_t_HV_X86_GDT_BASE,
    GDT_LIMIT = sys::hv_x86_reg_t_HV_X86_GDT_LIMIT,
    LDTR = sys::hv_x86_reg_t_HV_X86_LDTR,
    LDT_BASE = sys::hv_x86_reg_t_HV_X86_LDT_BASE,
    LDT_LIMIT = sys::hv_x86_reg_t_HV_X86_LDT_LIMIT,
    LDT_AR = sys::hv_x86_reg_t_HV_X86_LDT_AR,
    TR = sys::hv_x86_reg_t_HV_X86_TR,
    TSS_BASE = sys::hv_x86_reg_t_HV_X86_TSS_BASE,
    TSS_LIMIT = sys::hv_x86_reg_t_HV_X86_TSS_LIMIT,
    TSS_AR = sys::hv_x86_reg_t_HV_X86_TSS_AR,
    CR0 = sys::hv_x86_reg_t_HV_X86_CR0,
    CR1 = sys::hv_x86_reg_t_HV_X86_CR1,
    CR2 = sys::hv_x86_reg_t_HV_X86_CR2,
    CR3 = sys::hv_x86_reg_t_HV_X86_CR3,
    CR4 = sys::hv_x86_reg_t_HV_X86_CR4,
    DR0 = sys::hv_x86_reg_t_HV_X86_DR0,
    DR1 = sys::hv_x86_reg_t_HV_X86_DR1,
    DR2 = sys::hv_x86_reg_t_HV_X86_DR2,
    DR3 = sys::hv_x86_reg_t_HV_X86_DR3,
    DR4 = sys::hv_x86_reg_t_HV_X86_DR4,
    DR5 = sys::hv_x86_reg_t_HV_X86_DR5,
    DR6 = sys::hv_x86_reg_t_HV_X86_DR6,
    DR7 = sys::hv_x86_reg_t_HV_X86_DR7,
    TPR = sys::hv_x86_reg_t_HV_X86_TPR,
    XCR0 = sys::hv_x86_reg_t_HV_X86_XCR0,
    MAX = sys::hv_x86_reg_t_HV_X86_REGISTERS_MAX,
}
