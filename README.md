# simple_core

A minimal operating system kernel written in Rust

## Project Structure

- `kernel/` - Main kernel crate (core OS logic)
- `src/` - Additional binaries or entry points
- `build.rs` - Build script for custom build steps
- `Cargo.toml` - Workspace manifest

## Prerequisites

- Linux host
- Nightly Rust toolchain
- QEMU (for running the kernel)

## Running

```sh
cargo run
```
