# Toxoid Engine
*⚠️ Warning ⚠️ - This repository is under construction.*

# Required Dependencies

### Rust
https://www.rust-lang.org/tools/install

### Cargo Component
https://github.com/bytecodealliance/cargo-component

### LLVM
https://github.com/llvm/llvm-project/releases

### Emscripten (Web target only)
https://emscripten.org/docs/getting_started/downloads.html

# Examples
How to run Snake example:
```
make install
make build-host
make run-cli
```

# Notes
- Supported Spine Version - 4.1.24 ESS

# Credits
This technology would not be possible without:
- [Flecs](https://github.com/SanderMertens/flecs): A fast, lightweight (zero dependency), and portable entity component system written in C.
- [Sokol](https://github.com/floooh/sokol): Headers-only cross-platform C renderer with WebGPU support.
- [Wasmtime](https://github.com/bytecodealliance/wasmtime): A fast and secure JIT runtime for WebAssembly.
- [Wasmi](https://github.com/paritytech/wasmi): A fast and secure WebAssembly interpreter.
- [Emscripten](https://github.com/emscripten-core/emsdk): A complete compiler toolchain to WebAssembly, using LLVM, with a special focus on speed, size, and the Web platform.
- [Rust](https://www.rust-lang.org/): A memory-safe systems programming language. 
