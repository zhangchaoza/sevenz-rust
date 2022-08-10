This project is a 7z decompressor written in pure rust.

Only support lzma and lzma2 method currentlly

## Usage

Decompress source file "data/sample.7z" to dest path "data/sample"

```rust
sevenz_rust::decompress_file("data/sample.7z", "data/sample").expect("complete");
```

## Dependencies
- [crc](https://crates.io/crates/crc)
- [bit-set](https://crates.io/crates/bit-set)
- [lzma-rs](https://crates.io/crates/lzma-rs)
