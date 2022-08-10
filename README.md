This project is a 7z decompressor written in pure rust.

Only support lzma and lzma2 method currentlly

## Usage
```rust
sevenz_rust::decompress("data/sample.7z", "data/sample").expect("complete");
```