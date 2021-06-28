use crate::{call, sys, Error};

#[cfg(target_arch = "x86_64")]
type Handle = sys::hv_vcpuid_t;
#[cfg(target_arch = "aarch64")]
type Handle = sys::hv_vcpu_t;

/// Represents a single virtual CPU.
///
/// [Vcpu] object is not thread safe, all calls must be performed from
/// the owning thread.
pub struct Vcpu(pub(crate) Handle);

impl Vcpu {
    /// Creates a vCPU instance for the current thread.
    pub(crate) fn new() -> Result<Vcpu, Error> {
        let mut handle = 0;
        #[cfg(target_arch = "x86_64")]
        call!(sys::hv_vcpu_create(&mut handle, 0))?;
        Ok(Vcpu(handle))
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
}

impl Drop for Vcpu {
    /// Destroys the vCPU instance associated with the current thread.
    fn drop(&mut self) {
        let _ = call!(sys::hv_vcpu_destroy(self.0));
    }
}
