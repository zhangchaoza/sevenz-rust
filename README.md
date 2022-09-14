[![Crate](https://img.shields.io/crates/v/sevenz-rust.svg)](https://crates.io/crates/sevenz-rust)
 [![Documentation](https://docs.rs/sevenz-rust/badge.svg)](https://docs.rs/sevenz-rust)
 
This project is a 7z compressor/decompressor written in pure rust.<br/>
And it's very much inspired by the [apache commons-compress](https://commons.apache.org/proper/commons-compress/) project.

Only support lzma and lzma2 method currentlly

BCJ filter support is in progress

 - [x] X86
 - [ ] PPC
 - [ ] IA64
 - [ ] ARM
 - [ ] ARM_THUMB
 - [ ] SPARC
 
## Usage

Decompress source file "data/sample.7z" to dest path "data/sample"

```rust
sevenz_rust::decompress_file("data/sample.7z", "data/sample").expect("complete");
```

## Dependencies
- [crc](https://crates.io/crates/crc)
- [bit-set](https://crates.io/crates/bit-set)
- [lzma-rs](https://crates.io/crates/lzma-rs)
