use hv::x86::{Capability, VmExt, VmOptions};

fn main() -> Result<(), hv::Error> {
    hv::Vm::create(VmOptions::default())?;

    println!("Max vCPUs: {}", hv::Vm::capability(Capability::VcpuMax)?);

    println!(
        "Available address spaces: {}",
        hv::Vm::capability(Capability::AddrSpaceMax)?
    );

    hv::Vm::destroy()?;

    Ok(())
}
