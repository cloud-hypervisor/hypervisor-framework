use crate::{call, sys, Error};

/// Represents a single virtual CPU.
///
/// [Vcpu] object is not thread safe, all calls must be performed from
/// the owning thread.
pub struct Vcpu {
    #[cfg(target_arch = "x86_64")]
    pub(crate) cpu: sys::hv_vcpuid_t,
    #[cfg(target_arch = "aarch64")]
    pub(crate) cpu: sys::hv_vcpu_t,
    #[cfg(target_arch = "aarch64")]
    /// The pointer to the vCPU exit information.
    /// The function `hv_vcpu_run` updates this structure on return.
    /// Apple silicon only.
    pub(crate) exit: *const sys::hv_vcpu_exit_t,
}

impl Vcpu {
    /// Creates a vCPU instance for the current thread.
    pub(crate) fn new() -> Result<Vcpu, Error> {
        #[cfg(target_arch = "x86_64")]
        {
            let mut cpu = 0;
            call!(sys::hv_vcpu_create(&mut cpu, 0))?;
            Ok(Vcpu { cpu })
        }

        #[cfg(target_arch = "aarch64")]
        {
            let mut cpu = 0;
            let mut exit = std::ptr::null_mut();
            call!(sys::hv_vcpu_create(
                &mut cpu,
                &mut exit,
                std::ptr::null_mut()
            ))?;
            Ok(Vcpu { cpu, exit })
        }
    }

    /// Executes a vCPU.
    ///
    /// Call blocks until the next exit of the vCPU [1].
    /// The owning thread must call this function.
    ///
    /// # Intel
    /// On an Intel-based Mac, `hv_vcpu_run` exits from causes external to the guest.
    /// To avoid the overhead of spurious exits use `hv_vcpu_run_until` with the deadline `HV_DEADLINE_FOREVER`.
    ///
    /// # Apple Silicon
    /// If the exit is of type `HV_EXIT_REASON_VTIMER_ACTIVATED`, the VTimer is automatically masked.
    /// As a result, no timer fires until the timer is unmasked with `hv_vcpu_set_vtimer_mask`.
    ///
    /// [1]: https://developer.apple.com/documentation/hypervisor/1441231-hv_vcpu_run
    pub fn run(&self) -> Result<(), Error> {
        call!(sys::hv_vcpu_run(self.cpu))
    }

    /// Returns the cumulative execution time of a vCPU in nanoseconds.
    pub fn exec_time(&self) -> Result<u64, Error> {
        let mut out = 0_u64;
        call!(sys::hv_vcpu_get_exec_time(self.cpu, &mut out))?;
        Ok(out)
    }
}

impl Drop for Vcpu {
    /// Destroys the vCPU instance associated with the current thread.
    fn drop(&mut self) {
        let _ = call!(sys::hv_vcpu_destroy(self.cpu));
    }
}
