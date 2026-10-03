# Rust dylib linkage regression

The shared crate compresses data inside a Rust `dylib`. The executable instantiates
the generic stream decoder outside that dylib and checks the decoded bytes.
This exercises the native symbol association required by Rust issue #65610.

Run from this directory:

```sh
cargo run
cargo run --features bindgen
cargo run --features cmake
cargo run --features cmake,bindgen
cargo run --features cmake,seekable
cargo run --features bindgen,seekable
```

The seekable cases also call native create/free functions from the executable.
CMake builds those functions into a separate archive.

For a negative control, `cargo run --no-default-features` must fail to link on a
compiler affected by #65610. If it succeeds, check whether the compiler fixed the
issue before treating a passing positive test as evidence for the workaround.

To exercise pkg-config selection with the CMake feature enabled, set
`ZSTD_SYS_USE_PKG_CONFIG=1` and point `PKG_CONFIG_PATH` at a libzstd installation
containing its static archive, then run `cargo run --features cmake,bindgen`.
