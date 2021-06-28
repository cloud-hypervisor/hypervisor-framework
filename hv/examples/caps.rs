#[cfg(target_arch = "x86_64")]
fn main() -> Result<(), hv::Error> {
    use hv::x86::{Capability, VmExt, VmOptions};

    hv::Vm::create_vm(VmOptions::default())?;

    println!("Max vCPUs: {}", hv::Vm::capability(Capability::VcpuMax)?);

    println!(
        "Available address spaces: {}",
        hv::Vm::capability(Capability::AddrSpaceMax)?
    );

    hv::Vm::destroy()?;

    Ok(())
}

#[cfg(target_arch = "aarch64")]
fn main() {}
