use sevenz_rust::*;
use std::collections::HashMap;
use tempfile::*;

#[cfg(feature = "compress")]
#[test]
fn compress_decompress_anti_items() -> Result<(), Error> {
    let temp_dir = tempdir()?;
    let dest = temp_dir.path().join("anti.7z");
    let mut is_anti_map = HashMap::new();

    let mut sz = SevenZWriter::create(&dest)?;

    // a not anti file
    let mut e = SevenZArchiveEntry::default();
    e.name = "not_anti.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = false;
    is_anti_map.insert(e.name.to_string(), false);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a anti
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a not anti directory
    let mut e = SevenZArchiveEntry::default();
    e.name = "not_anti".to_string();
    e.has_stream = false;
    e.is_directory = true;
    e.is_anti_item = false;
    is_anti_map.insert(e.name.to_string(), false);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a anti directory
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti".to_string();
    e.has_stream = false;
    e.is_directory = true;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a anti directory with anti file
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti_with_anti_file".to_string();
    e.has_stream = false;
    e.is_directory = true;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti_with_anti_file/anti.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a anti directory with not anti file
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti_with_not_anti_file".to_string();
    e.has_stream = false;
    e.is_directory = true;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti_with_not_anti_file/not_anti.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = false;
    is_anti_map.insert(e.name.to_string(), false);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a not anti directory with anti file
    let mut e = SevenZArchiveEntry::default();
    e.name = "not_anti_with_anti_file".to_string();
    e.has_stream = false;
    e.is_directory = true;
    e.is_anti_item = false;
    is_anti_map.insert(e.name.to_string(), false);
    sz.push_archive_entry::<&[u8]>(e, None)?;
    let mut e = SevenZArchiveEntry::default();
    e.name = "not_anti_with_anti_file/anti.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), true);
    sz.push_archive_entry::<&[u8]>(e, None)?;

    // a anti file with contents
    let mut e = SevenZArchiveEntry::default();
    e.name = "anti_with_contents.txt".to_string();
    e.has_stream = false;
    e.is_directory = false;
    e.is_anti_item = true;
    is_anti_map.insert(e.name.to_string(), false);
    sz.push_archive_entry(e, Some(b"hello world".as_slice()))?;

    sz.finish()?;

    // decompression and check is_anti field
    let dir = temp_dir.path().join("output");
    decompress_file(&dest, &dir)?;
    assert_eq!(
        true,
        temp_dir.path().join("output").join("not_anti.txt").exists()
    );
    assert_eq!(
        false,
        temp_dir.path().join("output").join("anti.txt").exists()
    );
    assert_eq!(
        true,
        temp_dir.path().join("output").join("not_anti").exists()
    );
    assert_eq!(false, temp_dir.path().join("output").join("anti").exists());
    assert_eq!(
        false,
        temp_dir
            .path()
            .join("output")
            .join("anti_with_anti_file")
            .exists()
    );
    assert_eq!(
        false,
        temp_dir
            .path()
            .join("output")
            .join("anti_with_anti_file")
            .join("anti.txt")
            .exists()
    );
    assert_eq!(
        true,
        temp_dir
            .path()
            .join("output")
            .join("anti_with_not_anti_file")
            .exists()
    );
    assert_eq!(
        true,
        temp_dir
            .path()
            .join("output")
            .join("anti_with_not_anti_file")
            .join("not_anti.txt")
            .exists()
    );
    assert_eq!(
        true,
        temp_dir
            .path()
            .join("output")
            .join("not_anti_with_anti_file")
            .exists()
    );
    assert_eq!(
        false,
        temp_dir
            .path()
            .join("output")
            .join("not_anti_with_anti_file")
            .join("anti.txt")
            .exists()
    );
    assert_eq!(
        true,
        temp_dir
            .path()
            .join("output")
            .join("anti_with_contents.txt")
            .exists()
    );

    // check reader
    let mut sz = SevenZReader::open(&dest, Password::empty())?;
    sz.for_each_entries(|e, _r| {
        assert_eq!(is_anti_map.get(&e.name).unwrap(), &e.is_anti_item);
        Ok(true)
    })?;

    Ok(())
}
