# hv-sys

[![CI](https://github.com/cloud-hypervisor/hypervisor-framework/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/cloud-hypervisor/hypervisor-framework/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/hv-sys)](https://crates.io/crates/hv-sys)
![Crates.io](https://img.shields.io/crates/l/hv-sys)

Unsafe `-sys` crate with raw, unsafe bindings for [Hypervisor Framework](https://developer.apple.com/documentation/hypervisor), automatically generated with `bindgen`.

Please don't use this crate directly, have a look on [hv](https://crates.io/crates/hv) crate instead.
It offers high level safer Rust API to access Hypervisor Framework.

Also please see the [repository](https://github.com/cloud-hypervisor/hypervisor-framework) for ongoing work, questions, submit bugs, etc.

## Usage

Add the following to your `Cargo.toml`:
```toml
[dependencies]
hv-sys = "0.1"
```
