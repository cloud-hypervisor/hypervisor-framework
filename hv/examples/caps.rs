#[cfg(target_arch = "x86_64")]
fn main() -> Result<(), hv::Error> {
    use hv::x86::{Capability, VmExt, VmOptions};

    let vm = hv::Vm::new(VmOptions::default())?;

    println!("Max vCPUs: {}", vm.capability(Capability::VcpuMax)?);

    println!(
        "Available address spaces: {}",
        vm.capability(Capability::AddrSpaceMax)?
    );

    Ok(())
}

#[cfg(target_arch = "aarch64")]
fn main() {}
