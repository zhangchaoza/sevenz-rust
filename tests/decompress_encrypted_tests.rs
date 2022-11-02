use std::path::PathBuf;
use tempfile::tempdir;
use sevenz_rust::*;

#[cfg(feature = "aes256")]
#[test]
fn test_decompress_file_with_password(){
    let mut source_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    source_file.push("tests/resources/encrypted.7z");
    let temp_dir = tempdir().unwrap();
    let target = temp_dir.path().to_path_buf();
    let r = decompress_file_with_password(source_file, target.as_path(), "sevenz-rust".into());
    assert!(r.is_ok())
}