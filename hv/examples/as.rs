// Apple Silicon example.
// Adapted from https://github.com/zhuowei/FakeHVF/blob/main/simplevm.c

use std::ptr;

use hv::arm64::{Reg, VcpuExt};

static CODE: &[u8] = &[
    // Compute ((2 + 2) - 1)
    0x40, 0x00, 0x80, 0xD2, // mov x0, #2
    0x00, 0x08, 0x00, 0x91, // add x0, x0, #2
    0x00, 0x04, 0x00, 0xD1, // sub x0, x0, #1
    // Write it to memory pointed by x1
    0x20, 0x00, 0x00, 0xF9, // str x0, [x1]
    // Reboot the computer with PSCI/SMCCC
    // 0x84000009 is PSCI SYSTEM_RESET using SMC32 calling convention
    0x20, 0x01, 0x80, 0xd2, // mov x0, 0x0009
    0x00, 0x80, 0xb0, 0xf2, // movk x0, 0x8400, lsl #16
    0x02, 0x00, 0x00, 0xD4, // hvc #0
    // Infinite loop
    0x00, 0x00, 0x00, 0x14,
];

const MEM_SIZE: usize = 0x100000;
const GUEST_ADDR: usize = 0x69420000;

const RESULT_OFFSET: usize = 0x100;
const GUEST_RESULT_ADDR: usize = GUEST_ADDR + RESULT_OFFSET;

fn main() -> Result<(), hv::Error> {
    let load_addr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            MEM_SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE | libc::MAP_NORESERVE,
            -1,
            0,
        ) as *mut u8
    };

    if load_addr == libc::MAP_FAILED as _ {
        panic!("libc::mmap returned MAP_FAILED");
    }

    unsafe {
        ptr::copy_nonoverlapping(CODE.as_ptr(), load_addr, CODE.len());
    }

    // Init VM
    hv::Vm::create_vm(ptr::null_mut()).expect("Failed to create VM");

    // Initialize guest memory
    hv::Vm::map(
        load_addr,
        GUEST_ADDR as _,
        MEM_SIZE as _,
        hv::Memory::READ | hv::Memory::WRITE | hv::Memory::EXEC,
    )
    .expect("Failed to map guest memory");

    // Create VCPU
    let cpu = hv::Vm::create_cpu().expect("Failed to create CPU");

    // Register regs
    cpu.set_reg(Reg::PC, GUEST_ADDR as _)
        .expect("Failed to set PC reg");

    cpu.set_reg(Reg::X1, GUEST_RESULT_ADDR as _)
        .expect("Failed to set X1");

    loop {
        cpu.run().expect("Failed to run CPU");

        let info = cpu.exit_info();
        println!("{:?}", info);

        break;
    }

    let result_addr = unsafe { load_addr.add(RESULT_OFFSET) } as *const u64;
    let result = unsafe { *result_addr };

    println!("Result: {}", result);
    assert_eq!(result, 3);

    drop(cpu);

    hv::Vm::destroy()?;

    Ok(())
}
