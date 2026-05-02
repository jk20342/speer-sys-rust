//! Raw bindgen-generated bindings for libspeer (`speer.h` and optional expanded headers).
//!
//! Typical usage: depend on [`speer`](https://docs.rs/speer/latest/speer/) for the safe wrapper and
//! only pull [`crate`] types/functions when you need `#[repr(C)]` layouts or unchecked FFI glue.
//!
//! With the default `build-from-source` Cargo feature this crate invokes CMake so the bindings
//! link against freshly built artifact paths exposed through `cargo:rustc-link-*` directives from
//! `build.rs`.
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
