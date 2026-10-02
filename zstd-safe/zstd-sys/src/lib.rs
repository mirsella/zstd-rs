#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
//! Low-level bindings to the [zstd] library.
//!
//! [zstd]: https://facebook.github.io/zstd/

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
mod wasm_shim;

// The build script selects pregenerated bindings or runs bindgen, and attaches native
// library metadata when these bindings will cross a Rust dylib boundary.
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
