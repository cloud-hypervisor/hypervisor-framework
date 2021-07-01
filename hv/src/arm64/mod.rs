//! Apple Silicon extensions support.

use crate::{call, sys, Error, Vcpu};

mod regs;
pub use regs::*;

/// Injected interrupt type.
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InterruptType {
    IRQ = sys::hv_interrupt_type_t_HV_INTERRUPT_TYPE_IRQ,
    FIQ = sys::hv_interrupt_type_t_HV_INTERRUPT_TYPE_FIQ,
}

/// Contains information about an exit from the vcpu to the host.
pub type VcpuExit = sys::hv_vcpu_exit_t;

pub trait VcpuExt {
    /// Returns the current value of a vCPU register.
    fn get_reg(&self, reg: regs::Reg) -> Result<u64, Error>;

    /// Sets the value of a vCPU register.
    fn set_reg(&self, reg: regs::Reg, value: u64) -> Result<(), Error>;

    /// Returns the current value of a vCPU SIMD & FP register.
    fn get_simd_fp_reg(&self, reg: regs::SimdFpReg) -> Result<regs::SimdFpUchar16, Error>;

    /// Sets the value of a vCPU SIMD & FP register
    fn set_simd_fp_reg(
        &self,
        reg: regs::SimdFpReg,
        value: regs::SimdFpUchar16,
    ) -> Result<(), Error>;

    /// Returns the current value of a vCPU system register.
    fn get_sys_reg(&self, reg: regs::SysReg) -> Result<u64, Error>;

    /// Sets the value of a vCPU system register.
    fn set_sys_reg(&self, reg: regs::SysReg, value: u64) -> Result<(), Error>;

    /// Gets pending interrupts for a vcpu.
    fn pending_interrupt(&self, ty: InterruptType) -> Result<bool, Error>;

    /// Sets pending interrupts for a vcpu.
    fn set_pending_interrupt(&self, ty: InterruptType, pending: bool) -> Result<(), Error>;

    /// Get whether debug exceptions in the guest are trapped to the host.
    fn trap_debug_exceptions(&self) -> Result<bool, Error>;

    /// Set whether debug exceptions in the guest are trapped to the host.
    fn set_trap_debug_exceptions(&self, enable: bool) -> Result<(), Error>;

    /// Get whether debug register accesses in the guest are trapped to the host.
    fn trap_debug_reg_accesses(&self) -> Result<bool, Error>;

    /// Set whether debug register accesses in the guest are trapped to the host.
    fn set_trap_debug_reg_accesses(&self, enable: bool) -> Result<(), Error>;

    /// Gets the VTimer mask.
    fn vtimer_mask(&self) -> Result<bool, Error>;

    /// Sets the VTimer mask.
    fn set_vtimer_mask(&self, vtimer_is_masked: bool) -> Result<(), Error>;

    /// Gets the VTimer offset.
    fn vtimer_offset(&self) -> Result<u64, Error>;

    /// Sets the VTimer offset.
    fn set_vtimer_offset(&self, vtimer_offset: u64) -> Result<(), Error>;

    /// Returns the underlying `hv_vcpu_exit_t` structure.
    fn exit_info(&self) -> VcpuExit;
}

impl VcpuExt for Vcpu {
    /// Returns the current value of a vCPU register.
    fn get_reg(&self, reg: regs::Reg) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vcpu_get_reg(self.id, reg as _, &mut out))?;
        Ok(out)
    }

    /// Sets the value of a vCPU register.
    fn set_reg(&self, reg: regs::Reg, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_reg(self.id, reg as _, value))
    }

    /// Returns the current value of a vCPU SIMD & FP register.
    fn get_simd_fp_reg(&self, reg: regs::SimdFpReg) -> Result<regs::SimdFpUchar16, Error> {
        let mut out = 0_u128;
        call!(sys::hv_vcpu_get_simd_fp_reg(self.id, reg as _, &mut out))?;
        Ok(out)
    }

    /// Sets the value of a vCPU SIMD & FP register.
    fn set_simd_fp_reg(
        &self,
        reg: regs::SimdFpReg,
        value: regs::SimdFpUchar16,
    ) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_simd_fp_reg(self.id, reg as _, value))?;
        Ok(())
    }

    /// Returns the current value of a vCPU system register.
    fn get_sys_reg(&self, reg: regs::SysReg) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vcpu_get_sys_reg(self.id, reg as _, &mut out))?;
        Ok(out)
    }

    /// Sets the value of a vCPU system register.
    fn set_sys_reg(&self, reg: regs::SysReg, value: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_sys_reg(self.id, reg as _, value))
    }

    /// Gets pending interrupts for a vcpu.
    fn pending_interrupt(&self, ty: InterruptType) -> Result<bool, Error> {
        let mut out = false;
        call!(sys::hv_vcpu_get_pending_interrupt(
            self.id, ty as u32, &mut out
        ))?;
        Ok(out)
    }

    /// Sets pending interrupts for a vcpu.
    fn set_pending_interrupt(&self, ty: InterruptType, mut pending: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_get_pending_interrupt(
            self.id,
            ty as u32,
            &mut pending
        ))
    }

    /// Get whether debug exceptions in the guest are trapped to the host.
    fn trap_debug_exceptions(&self) -> Result<bool, Error> {
        let mut out = false;
        call!(sys::hv_vcpu_get_trap_debug_exceptions(self.id, &mut out))?;
        Ok(out)
    }

    /// Set whether debug exceptions in the guest are trapped to the host.
    fn set_trap_debug_exceptions(&self, enable: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_trap_debug_exceptions(self.id, enable))
    }

    /// Get whether debug register accesses in the guest are trapped to the host.
    fn trap_debug_reg_accesses(&self) -> Result<bool, Error> {
        let mut out = false;
        call!(sys::hv_vcpu_get_trap_debug_reg_accesses(self.id, &mut out))?;
        Ok(out)
    }

    /// Set whether debug register accesses in the guest are trapped to the host.
    fn set_trap_debug_reg_accesses(&self, enable: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_trap_debug_reg_accesses(self.id, enable))
    }

    /// Gets the VTimer mask.
    fn vtimer_mask(&self) -> Result<bool, Error> {
        let mut out = false;
        call!(sys::hv_vcpu_get_vtimer_mask(self.id, &mut out))?;
        Ok(out)
    }

    /// Sets the VTimer mask.
    fn set_vtimer_mask(&self, vtimer_is_masked: bool) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_vtimer_mask(self.id, vtimer_is_masked))
    }

    /// Gets the VTimer offset.
    fn vtimer_offset(&self) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vcpu_get_vtimer_offset(self.id, &mut out))?;
        Ok(out)
    }

    /// Sets the VTimer offset.
    fn set_vtimer_offset(&self, vtimer_offset: u64) -> Result<(), Error> {
        call!(sys::hv_vcpu_set_vtimer_offset(self.id, vtimer_offset))
    }

    /// Returns the underlying `hv_vcpu_exit_t` structure.
    fn exit_info(&self) -> VcpuExit {
        if self.exit.is_null() {
            VcpuExit::default()
        } else {
            unsafe { *self.exit }
        }
    }
}
