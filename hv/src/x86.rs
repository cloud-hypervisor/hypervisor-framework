//! x86 specific routines.

use crate::{sys, Error};

/// x86 specific routines for vCPU.
pub trait VCpuX86Ext {
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

/// x86 architectural register IDs.
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
