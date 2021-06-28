fn main() {
    hv::vm_create().unwrap();

    println!(
        "Max vCPUs: {}",
        hv::capability(hv::Capability::VCpuMax).unwrap()
    );

    println!(
        "Available address spaces: {}",
        hv::capability(hv::Capability::AddrSpaceMax).unwrap()
    );

    hv::vm_destroy().unwrap();
}
