fn main() {
    hv::vm_create(hv::VmOptions::default()).unwrap();

    println!(
        "Max vCPUs: {}",
        hv::capability(hv::Capability::VCPU_MAX).unwrap()
    );

    println!(
        "Available address spaces: {}",
        hv::capability(hv::Capability::ADDR_SPAC_EMAX).unwrap()
    );

    hv::vm_destroy().unwrap();
}
