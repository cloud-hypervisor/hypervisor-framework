//! VMX extensions.

use crate::{call, sys, Error, Vcpu};

/// Enum type of VMX cabability fields
#[repr(u32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Capability {
    /// Pin-based VMX capabilities.
    PinBased = sys::hv_vmx_capability_t_HV_VMX_CAP_PINBASED,
    /// Primary proc-based VMX capabilities.
    ProcBased = sys::hv_vmx_capability_t_HV_VMX_CAP_PROCBASED,
    /// Second proc-based VMX capabilities.
    ProcBased2 = sys::hv_vmx_capability_t_HV_VMX_CAP_PROCBASED2,
    /// VM-entry VMX capabilities.
    Entry = sys::hv_vmx_capability_t_HV_VMX_CAP_ENTRY,
    /// VM-exit VMX capabilities.
    Exit = sys::hv_vmx_capability_t_HV_VMX_CAP_EXIT,
    /// VMX preemption timer frequency.
    PreemptionTimer = sys::hv_vmx_capability_t_HV_VMX_CAP_PREEMPTION_TIMER,
}

/// Returns the VMX capabilities of the host processor.
pub fn read_capability(field: Capability) -> Result<u64, Error> {
    let mut out = 0_u64;
    call!(sys::hv_vmx_read_capability(field as u32, &mut out))?;
    Ok(out)
}

bitflags::bitflags! {
    #[cfg(feature = "hv_10_15")]
    pub struct ShadowFlags: u32 {
        const NONE = sys::HV_SHADOW_VMCS_NONE;
        const READ = sys::HV_SHADOW_VMCS_READ;
        const WRITE = sys::HV_SHADOW_VMCS_WRITE;
    }
}

pub trait VCpuVmxExt {
    /// Returns the current value of a VMCS field of a vCPU.
    fn read_vmcs(&self, field: Vmcs) -> Result<u64, Error>;

    /// Set the value of a VMCS field of a vCPU.
    fn write_vmcs(&self, field: Vmcs, value: u64) -> Result<(), Error>;

    /// Returns the current value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn read_shadow_vmcs(&self, field: Vmcs) -> Result<u64, Error>;

    /// Set the value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn write_shadow_vmcs(&self, field: Vmcs, value: u64) -> Result<(), Error>;

    /// Set the access permissions of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn set_shadow_access(&self, field: Vmcs, flags: ShadowFlags) -> Result<(), Error>;
}

impl VCpuVmxExt for Vcpu {
    /// Returns the current value of a VMCS field of a vCPU.
    fn read_vmcs(&self, field: Vmcs) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vmx_vcpu_read_vmcs(self.id, field as u32, &mut out))?;
        Ok(out)
    }

    /// Set the value of a VMCS field of a vCPU.
    fn write_vmcs(&self, field: Vmcs, value: u64) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_write_vmcs(self.id, field as u32, value))
    }

    /// Returns the current value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn read_shadow_vmcs(&self, field: Vmcs) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vmx_vcpu_read_shadow_vmcs(
            self.id,
            field as u32,
            &mut out
        ))?;
        Ok(out)
    }

    /// Set the value of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn write_shadow_vmcs(&self, field: Vmcs, value: u64) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_write_shadow_vmcs(
            self.id,
            field as u32,
            value
        ))
    }

    /// Set the access permissions of a shadow VMCS field of a vCPU.
    #[cfg(feature = "hv_10_15")]
    fn set_shadow_access(&self, field: Vmcs, flags: ShadowFlags) -> Result<(), Error> {
        call!(sys::hv_vmx_vcpu_set_shadow_access(
            self.id,
            field as u32,
            flags.bits() as u64
        ))
    }
}

/// Virtual Machine Control Structure (VMCS) Field IDs.
/// Identify the fields of the virtual machine control structure.
#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Vmcs {
    VPID = sys::VMCS_VPID,
    CTRL_POSTED_INT_N_VECTOR = sys::VMCS_CTRL_POSTED_INT_N_VECTOR,
    CTRL_EPTP_INDEX = sys::VMCS_CTRL_EPTP_INDEX,
    GUEST_ES = sys::VMCS_GUEST_ES,
    GUEST_CS = sys::VMCS_GUEST_CS,
    GUEST_SS = sys::VMCS_GUEST_SS,
    GUEST_DS = sys::VMCS_GUEST_DS,
    GUEST_FS = sys::VMCS_GUEST_FS,
    GUEST_GS = sys::VMCS_GUEST_GS,
    GUEST_LDTR = sys::VMCS_GUEST_LDTR,
    GUEST_TR = sys::VMCS_GUEST_TR,
    GUEST_INT_STATUS = sys::VMCS_GUEST_INT_STATUS,
    GUESTPML_INDEX = sys::VMCS_GUESTPML_INDEX,
    HOST_ES = sys::VMCS_HOST_ES,
    HOST_CS = sys::VMCS_HOST_CS,
    HOST_SS = sys::VMCS_HOST_SS,
    HOST_DS = sys::VMCS_HOST_DS,
    HOST_FS = sys::VMCS_HOST_FS,
    HOST_GS = sys::VMCS_HOST_GS,
    HOST_TR = sys::VMCS_HOST_TR,
    CTRL_IO_BITMAP_A = sys::VMCS_CTRL_IO_BITMAP_A,
    CTRL_IO_BITMAP_B = sys::VMCS_CTRL_IO_BITMAP_B,
    CTRL_MSR_BITMAPS = sys::VMCS_CTRL_MSR_BITMAPS,
    CTRL_VMEXIT_MSR_STORE_ADDR = sys::VMCS_CTRL_VMEXIT_MSR_STORE_ADDR,
    CTRL_VMEXIT_MSR_LOAD_ADDR = sys::VMCS_CTRL_VMEXIT_MSR_LOAD_ADDR,
    CTRL_VMENTRY_MSR_LOAD_ADDR = sys::VMCS_CTRL_VMENTRY_MSR_LOAD_ADDR,
    CTRL_EXECUTIVE_VMCS_PTR = sys::VMCS_CTRL_EXECUTIVE_VMCS_PTR,
    CTRL_PML_ADDR = sys::VMCS_CTRL_PML_ADDR,
    CTRL_TSC_OFFSET = sys::VMCS_CTRL_TSC_OFFSET,
    CTRL_VIRTUAL_APIC = sys::VMCS_CTRL_VIRTUAL_APIC,
    CTRL_APIC_ACCESS = sys::VMCS_CTRL_APIC_ACCESS,
    CTRL_POSTED_INT_DESC_ADDR = sys::VMCS_CTRL_POSTED_INT_DESC_ADDR,
    CTRL_VMFUNC_CTRL = sys::VMCS_CTRL_VMFUNC_CTRL,
    CTRL_EPTP = sys::VMCS_CTRL_EPTP,
    CTRL_EOI_EXIT_BITMAP_0 = sys::VMCS_CTRL_EOI_EXIT_BITMAP_0,
    CTRL_EOI_EXIT_BITMAP_1 = sys::VMCS_CTRL_EOI_EXIT_BITMAP_1,
    CTRL_EOI_EXIT_BITMAP_2 = sys::VMCS_CTRL_EOI_EXIT_BITMAP_2,
    CTRL_EOI_EXIT_BITMAP_3 = sys::VMCS_CTRL_EOI_EXIT_BITMAP_3,
    CTRL_EPTP_LIST_ADDR = sys::VMCS_CTRL_EPTP_LIST_ADDR,
    CTRL_VMREAD_BITMAP_ADDR = sys::VMCS_CTRL_VMREAD_BITMAP_ADDR,
    CTRL_VMWRITE_BITMAP_ADDR = sys::VMCS_CTRL_VMWRITE_BITMAP_ADDR,
    CTRL_VIRT_EXC_INFO_ADDR = sys::VMCS_CTRL_VIRT_EXC_INFO_ADDR,
    CTRL_XSS_EXITING_BITMAP = sys::VMCS_CTRL_XSS_EXITING_BITMAP,
    CTRL_ENCLS_EXITING_BITMAP = sys::VMCS_CTRL_ENCLS_EXITING_BITMAP,
    CTRL_TSC_MULTIPLIER = sys::VMCS_CTRL_TSC_MULTIPLIER,
    GUEST_PHYSICAL_ADDRESS = sys::VMCS_GUEST_PHYSICAL_ADDRESS,
    GUEST_LINK_POINTER = sys::VMCS_GUEST_LINK_POINTER,
    GUEST_IA32_DEBUGCTL = sys::VMCS_GUEST_IA32_DEBUGCTL,
    GUEST_IA32_PAT = sys::VMCS_GUEST_IA32_PAT,
    GUEST_IA32_EFER = sys::VMCS_GUEST_IA32_EFER,
    GUEST_IA32_PERF_GLOBAL_CTRL = sys::VMCS_GUEST_IA32_PERF_GLOBAL_CTRL,
    GUEST_PDPTE0 = sys::VMCS_GUEST_PDPTE0,
    GUEST_PDPTE1 = sys::VMCS_GUEST_PDPTE1,
    GUEST_PDPTE2 = sys::VMCS_GUEST_PDPTE2,
    GUEST_PDPTE3 = sys::VMCS_GUEST_PDPTE3,
    GUEST_IA32_BNDCFGS = sys::VMCS_GUEST_IA32_BNDCFGS,
    HOST_IA32_PAT = sys::VMCS_HOST_IA32_PAT,
    HOST_IA32_EFER = sys::VMCS_HOST_IA32_EFER,
    HOST_IA32_PERF_GLOBAL_CTRL = sys::VMCS_HOST_IA32_PERF_GLOBAL_CTRL,
    CTRL_PIN_BASED = sys::VMCS_CTRL_PIN_BASED,
    CTRL_CPU_BASED = sys::VMCS_CTRL_CPU_BASED,
    CTRL_EXC_BITMAP = sys::VMCS_CTRL_EXC_BITMAP,
    CTRL_PF_ERROR_MASK = sys::VMCS_CTRL_PF_ERROR_MASK,
    CTRL_PF_ERROR_MATCH = sys::VMCS_CTRL_PF_ERROR_MATCH,
    CTRL_CR3_COUNT = sys::VMCS_CTRL_CR3_COUNT,
    CTRL_VMEXIT_CONTROLS = sys::VMCS_CTRL_VMEXIT_CONTROLS,
    CTRL_VMEXIT_MSR_STORE_COUNT = sys::VMCS_CTRL_VMEXIT_MSR_STORE_COUNT,
    CTRL_VMEXIT_MSR_LOAD_COUNT = sys::VMCS_CTRL_VMEXIT_MSR_LOAD_COUNT,
    CTRL_VMENTRY_CONTROLS = sys::VMCS_CTRL_VMENTRY_CONTROLS,
    CTRL_VMENTRY_MSR_LOAD_COUNT = sys::VMCS_CTRL_VMENTRY_MSR_LOAD_COUNT,
    CTRL_VMENTRY_IRQ_INFO = sys::VMCS_CTRL_VMENTRY_IRQ_INFO,
    CTRL_VMENTRY_EXC_ERROR = sys::VMCS_CTRL_VMENTRY_EXC_ERROR,
    CTRL_VMENTRY_INSTR_LEN = sys::VMCS_CTRL_VMENTRY_INSTR_LEN,
    CTRL_TPR_THRESHOLD = sys::VMCS_CTRL_TPR_THRESHOLD,
    CTRL_CPU_BASED2 = sys::VMCS_CTRL_CPU_BASED2,
    CTRL_PLE_GAP = sys::VMCS_CTRL_PLE_GAP,
    CTRL_PLE_WINDOW = sys::VMCS_CTRL_PLE_WINDOW,
    RO_INSTR_ERROR = sys::VMCS_RO_INSTR_ERROR,
    RO_EXIT_REASON = sys::VMCS_RO_EXIT_REASON,
    RO_VMEXIT_IRQ_INFO = sys::VMCS_RO_VMEXIT_IRQ_INFO,
    RO_VMEXIT_IRQ_ERROR = sys::VMCS_RO_VMEXIT_IRQ_ERROR,
    RO_IDT_VECTOR_INFO = sys::VMCS_RO_IDT_VECTOR_INFO,
    RO_IDT_VECTOR_ERROR = sys::VMCS_RO_IDT_VECTOR_ERROR,
    RO_VMEXIT_INSTR_LEN = sys::VMCS_RO_VMEXIT_INSTR_LEN,
    RO_VMX_INSTR_INFO = sys::VMCS_RO_VMX_INSTR_INFO,
    GUEST_ES_LIMIT = sys::VMCS_GUEST_ES_LIMIT,
    GUEST_CS_LIMIT = sys::VMCS_GUEST_CS_LIMIT,
    GUEST_SS_LIMIT = sys::VMCS_GUEST_SS_LIMIT,
    GUEST_DS_LIMIT = sys::VMCS_GUEST_DS_LIMIT,
    GUEST_FS_LIMIT = sys::VMCS_GUEST_FS_LIMIT,
    GUEST_GS_LIMIT = sys::VMCS_GUEST_GS_LIMIT,
    GUEST_LDTR_LIMIT = sys::VMCS_GUEST_LDTR_LIMIT,
    GUEST_TR_LIMIT = sys::VMCS_GUEST_TR_LIMIT,
    GUEST_GDTR_LIMIT = sys::VMCS_GUEST_GDTR_LIMIT,
    GUEST_IDTR_LIMIT = sys::VMCS_GUEST_IDTR_LIMIT,
    GUEST_ES_AR = sys::VMCS_GUEST_ES_AR,
    GUEST_CS_AR = sys::VMCS_GUEST_CS_AR,
    GUEST_SS_AR = sys::VMCS_GUEST_SS_AR,
    GUEST_DS_AR = sys::VMCS_GUEST_DS_AR,
    GUEST_FS_AR = sys::VMCS_GUEST_FS_AR,
    GUEST_GS_AR = sys::VMCS_GUEST_GS_AR,
    GUEST_LDTR_AR = sys::VMCS_GUEST_LDTR_AR,
    GUEST_TR_AR = sys::VMCS_GUEST_TR_AR,
    GUEST_IGNORE_IRQ = sys::VMCS_GUEST_IGNORE_IRQ,
    GUEST_ACTIVITY_STATE = sys::VMCS_GUEST_ACTIVITY_STATE,
    GUEST_SMBASE = sys::VMCS_GUEST_SMBASE,
    GUEST_IA32_SYSENTER_CS = sys::VMCS_GUEST_IA32_SYSENTER_CS,
    GUEST_VMX_TIMER_VALUE = sys::VMCS_GUEST_VMX_TIMER_VALUE,
    HOST_IA32_SYSENTER_CS = sys::VMCS_HOST_IA32_SYSENTER_CS,
    CTRL_CR0_MASK = sys::VMCS_CTRL_CR0_MASK,
    CTRL_CR4_MASK = sys::VMCS_CTRL_CR4_MASK,
    CTRL_CR0_SHADOW = sys::VMCS_CTRL_CR0_SHADOW,
    CTRL_CR4_SHADOW = sys::VMCS_CTRL_CR4_SHADOW,
    CTRL_CR3_VALUE0 = sys::VMCS_CTRL_CR3_VALUE0,
    CTRL_CR3_VALUE1 = sys::VMCS_CTRL_CR3_VALUE1,
    CTRL_CR3_VALUE2 = sys::VMCS_CTRL_CR3_VALUE2,
    CTRL_CR3_VALUE3 = sys::VMCS_CTRL_CR3_VALUE3,
    RO_EXIT_QUALIFIC = sys::VMCS_RO_EXIT_QUALIFIC,
    RO_IO_RCX = sys::VMCS_RO_IO_RCX,
    RO_IO_RSI = sys::VMCS_RO_IO_RSI,
    RO_IO_RDI = sys::VMCS_RO_IO_RDI,
    RO_IO_RIP = sys::VMCS_RO_IO_RIP,
    RO_GUEST_LIN_ADDR = sys::VMCS_RO_GUEST_LIN_ADDR,
    GUEST_CR0 = sys::VMCS_GUEST_CR0,
    GUEST_CR3 = sys::VMCS_GUEST_CR3,
    GUEST_CR4 = sys::VMCS_GUEST_CR4,
    GUEST_ES_BASE = sys::VMCS_GUEST_ES_BASE,
    GUEST_CS_BASE = sys::VMCS_GUEST_CS_BASE,
    GUEST_SS_BASE = sys::VMCS_GUEST_SS_BASE,
    GUEST_DS_BASE = sys::VMCS_GUEST_DS_BASE,
    GUEST_FS_BASE = sys::VMCS_GUEST_FS_BASE,
    GUEST_GS_BASE = sys::VMCS_GUEST_GS_BASE,
    GUEST_LDTR_BASE = sys::VMCS_GUEST_LDTR_BASE,
    GUEST_TR_BASE = sys::VMCS_GUEST_TR_BASE,
    GUEST_GDTR_BASE = sys::VMCS_GUEST_GDTR_BASE,
    GUEST_IDTR_BASE = sys::VMCS_GUEST_IDTR_BASE,
    GUEST_DR7 = sys::VMCS_GUEST_DR7,
    GUEST_RSP = sys::VMCS_GUEST_RSP,
    GUEST_RIP = sys::VMCS_GUEST_RIP,
    GUEST_RFLAGS = sys::VMCS_GUEST_RFLAGS,
    GUEST_DEBUG_EXC = sys::VMCS_GUEST_DEBUG_EXC,
    GUEST_SYSENTER_ESP = sys::VMCS_GUEST_SYSENTER_ESP,
    GUEST_SYSENTER_EIP = sys::VMCS_GUEST_SYSENTER_EIP,
    HOST_CR0 = sys::VMCS_HOST_CR0,
    HOST_CR3 = sys::VMCS_HOST_CR3,
    HOST_CR4 = sys::VMCS_HOST_CR4,
    HOST_FS_BASE = sys::VMCS_HOST_FS_BASE,
    HOST_GS_BASE = sys::VMCS_HOST_GS_BASE,
    HOST_TR_BASE = sys::VMCS_HOST_TR_BASE,
    HOST_GDTR_BASE = sys::VMCS_HOST_GDTR_BASE,
    HOST_IDTR_BASE = sys::VMCS_HOST_IDTR_BASE,
    HOST_IA32_SYSENTER_ESP = sys::VMCS_HOST_IA32_SYSENTER_ESP,
    HOST_IA32_SYSENTER_EIP = sys::VMCS_HOST_IA32_SYSENTER_EIP,
    HOST_RSP = sys::VMCS_HOST_RSP,
    HOST_RIP = sys::VMCS_HOST_RIP,
    MAX = sys::VMCS_MAX,
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reason {
    EXC_NMI = sys::VMX_REASON_EXC_NMI,
    IRQ = sys::VMX_REASON_IRQ,
    TRIPLE_FAULT = sys::VMX_REASON_TRIPLE_FAULT,
    INIT = sys::VMX_REASON_INIT,
    SIPI = sys::VMX_REASON_SIPI,
    IO_SMI = sys::VMX_REASON_IO_SMI,
    OTHER_SMI = sys::VMX_REASON_OTHER_SMI,
    IRQ_WND = sys::VMX_REASON_IRQ_WND,
    VIRTUAL_NMI_WND = sys::VMX_REASON_VIRTUAL_NMI_WND,
    TASK = sys::VMX_REASON_TASK,
    CPUID = sys::VMX_REASON_CPUID,
    GETSEC = sys::VMX_REASON_GETSEC,
    HLT = sys::VMX_REASON_HLT,
    INVD = sys::VMX_REASON_INVD,
    INVLPG = sys::VMX_REASON_INVLPG,
    RDPMC = sys::VMX_REASON_RDPMC,
    RDTSC = sys::VMX_REASON_RDTSC,
    RSM = sys::VMX_REASON_RSM,
    VMCALL = sys::VMX_REASON_VMCALL,
    VMCLEAR = sys::VMX_REASON_VMCLEAR,
    VMLAUNCH = sys::VMX_REASON_VMLAUNCH,
    VMPTRLD = sys::VMX_REASON_VMPTRLD,
    VMPTRST = sys::VMX_REASON_VMPTRST,
    VMREAD = sys::VMX_REASON_VMREAD,
    VMRESUME = sys::VMX_REASON_VMRESUME,
    VMWRITE = sys::VMX_REASON_VMWRITE,
    VMOFF = sys::VMX_REASON_VMOFF,
    VMON = sys::VMX_REASON_VMON,
    MOV_CR = sys::VMX_REASON_MOV_CR,
    MOV_DR = sys::VMX_REASON_MOV_DR,
    IO = sys::VMX_REASON_IO,
    RDMSR = sys::VMX_REASON_RDMSR,
    WRMSR = sys::VMX_REASON_WRMSR,
    VMENTRY_GUEST = sys::VMX_REASON_VMENTRY_GUEST,
    VMENTRY_MSR = sys::VMX_REASON_VMENTRY_MSR,
    MWAIT = sys::VMX_REASON_MWAIT,
    MTF = sys::VMX_REASON_MTF,
    MONITOR = sys::VMX_REASON_MONITOR,
    PAUSE = sys::VMX_REASON_PAUSE,
    VMENTRY_MC = sys::VMX_REASON_VMENTRY_MC,
    TPR_THRESHOLD = sys::VMX_REASON_TPR_THRESHOLD,
    APIC_ACCESS = sys::VMX_REASON_APIC_ACCESS,
    VIRTUALIZED_EOI = sys::VMX_REASON_VIRTUALIZED_EOI,
    GDTR_IDTR = sys::VMX_REASON_GDTR_IDTR,
    LDTR_TR = sys::VMX_REASON_LDTR_TR,
    EPT_VIOLATION = sys::VMX_REASON_EPT_VIOLATION,
    EPT_MISCONFIG = sys::VMX_REASON_EPT_MISCONFIG,
    EPT_INVEPT = sys::VMX_REASON_EPT_INVEPT,
    RDTSCP = sys::VMX_REASON_RDTSCP,
    VMX_TIMER_EXPIRED = sys::VMX_REASON_VMX_TIMER_EXPIRED,
    INVVPID = sys::VMX_REASON_INVVPID,
    WBINVD = sys::VMX_REASON_WBINVD,
    XSETBV = sys::VMX_REASON_XSETBV,
    APIC_WRITE = sys::VMX_REASON_APIC_WRITE,
    RDRAND = sys::VMX_REASON_RDRAND,
    INVPCID = sys::VMX_REASON_INVPCID,
    VMFUNC = sys::VMX_REASON_VMFUNC,
    RDSEED = sys::VMX_REASON_RDSEED,
    XSAVES = sys::VMX_REASON_XSAVES,
    XRSTORS = sys::VMX_REASON_XRSTORS,
}

#[allow(non_camel_case_types)]
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum IrqInfo {
    EXT_IRQ = sys::IRQ_INFO_EXT_IRQ,
    NMI = sys::IRQ_INFO_NMI,
    HARD_EXC = sys::IRQ_INFO_HARD_EXC,
    SOFT_IRQ = sys::IRQ_INFO_SOFT_IRQ,
    PRIV_SOFT_EXC = sys::IRQ_INFO_PRIV_SOFT_EXC,
    SOFT_EXC = sys::IRQ_INFO_SOFT_EXC,
    ERROR_VALID = sys::IRQ_INFO_ERROR_VALID,
    VALID = sys::IRQ_INFO_VALID,
}
