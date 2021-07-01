# hv

[![CI](https://github.com/mxpv/hv/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/mxpv/hv/actions/workflows/ci.yml)
[![GitHub](https://img.shields.io/github/license/mxpv/hv)](https://github.com/mxpv/hv/blob/main/LICENSE)
[![docs.rs](https://img.shields.io/docsrs/hv)](https://docs.rs/hv/)

`hv` is a high level Rust bindings for Hypervisor Framework.

[Apple Documentation](https://developer.apple.com/documentation/hypervisor)

Build virtualization solutions on top of a lightweight hypervisor using Rust:
- Full Hypervisor Framework support.
- Supports Apple Silicon.
- Safe Rust API.

This repository contains the following crates:
| Name | Description | Links |
| --- | --- | --- |
| [`hv-sys`](./hv-sys) | Unsafe bindings generated with bindgen | [![Crates.io](https://img.shields.io/crates/v/hv-sys)](https://crates.io/crates/hv-sys) |
| [`hv`](./hv) | High level API to access Hypervisor Framework | [![Crates.io](https://img.shields.io/crates/v/hv)](https://crates.io/crates/hv) |

### Current list of things to do:
- Make high level API safer.
- Expand documentation.
- Add more examples.

## Requirements

### Hypervisor Framework

At runtime, determine whether the Hypervisor APIs are available on a particular machine with the `sysctl`:

```bash
$ sysctl kern.hv_support
kern.hv_support: 1
```

In order to use Hypervisor API your app must have `com.apple.security.hypervisor` entitlement.
Refer to [example.entitlements](example.entitlements) for example of how entitlement file might look like.

Use the following command to self sign your binary for local development:

```bash
$ codesign --sign - --force --entitlements=example.entitlements ./binary
```

### Rust

Developed and tested on latest stable Rust (1.53.0+).

Be sure to have [Xcode](https://developer.apple.com/xcode/) installed and don't forget to `xcode-select --install`,
otherwise `bindgen` may fail to find Hypervisor headers.

## Example

Here is basic "Hello world" example on Apple Silicon:
```rust
// Init VM
let vm = Arc::new(hv::Vm::new(std::ptr::null_mut())?);

// Initialize guest memory
vm.map(load_addr, GUEST_ADDR, MEM_SIZE, hv::Memory::READ)?;

// Create VCPU
let cpu = vm.create_cpu()?;

// Set regs
cpu.set_reg(Reg::PC, GUEST_ADDR)?
cpu.set_reg(Reg::X1, GUEST_RESULT_ADDR)?

loop {
    cpu.run().expect("Failed to run CPU");

    let info = cpu.exit_info();
    println!("{:?}", info);

    break;
}
```
