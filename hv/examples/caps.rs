fn main() -> Result<(), hv::Error> {
    hv::Vm::create(hv::VmOptions::default())?;

    println!(
        "Max vCPUs: {}",
        hv::Vm::capability(hv::Capability::VCPU_MAX)?
    );

    println!(
        "Available address spaces: {}",
        hv::Vm::capability(hv::Capability::ADDR_SPAC_EMAX)?
    );

    hv::Vm::destroy()?;

    Ok(())
}
