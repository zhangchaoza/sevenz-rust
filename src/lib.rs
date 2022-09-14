pub(crate) mod archive;
mod bcj;
pub(crate) mod coders;
mod error;
pub(crate) mod folder;
mod lzma2_coder;
mod lzma_coder;
mod reader;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub use archive::SevenZArchiveEntry;
pub use error::Error;
pub use reader::SevenZReader;

use std::fs::File;
use std::path::PathBuf;

///
/// decompress a 7z file
/// #Example
/// ```no_run
/// sevenz_rust::decompress_file("sample.7z", "sample").expect("complete");
///
/// ```
///
pub fn decompress_file(src_path: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), Error> {
    let file = std::fs::File::open(src_path.as_ref())
        .map_err(|e| Error::file_open(e, src_path.as_ref().to_string_lossy().to_string()))?;
    decompress(file, dest)
}
/// decompress a source reader to [dest] path
pub fn decompress<R: Read + Seek>(mut src_reader: R, dest: impl AsRef<Path>) -> Result<(), Error> {
    let pos = src_reader.stream_position().map_err(Error::io)?;
    let len = src_reader.seek(SeekFrom::End(0)).map_err(Error::io)?;
    src_reader.seek(SeekFrom::Start(pos)).map_err(Error::io)?;
    let mut seven = SevenZReader::new(src_reader, len, &[])?;
    let dest = PathBuf::from(dest.as_ref());
    seven.for_each_entries(|entry, reader| {
        if entry.is_directory() {
            let dir = dest.join(entry.name());
            if !dir.exists() {
                std::fs::create_dir_all(&dir).map_err(Error::io)?;
            }
        } else {
            let path = dest.join(entry.name());
            path.parent().and_then(|p| {
                if !p.exists() {
                    std::fs::create_dir_all(p).ok()
                } else {
                    None
                }
            });
            let mut file = File::create(&path)
                .map_err(|e| Error::file_open(e, path.to_string_lossy().to_string()))?;
            if entry.size() > 0 {
                std::io::copy(reader, &mut file).map_err(Error::io)?;
            }
        }
        Ok(true)
    })?;

    Ok(())
}
