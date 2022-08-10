pub(crate) mod archive;
pub(crate) mod coders;
mod error;
pub(crate) mod folder;
mod lzma2_coder;
mod lzma_coder;
mod reader;
use std::path::Path;

pub use archive::SevenZArchiveEntry;
pub use error::Error;
pub use reader::*;

use std::fs::File;
use std::path::PathBuf;

///
/// decompress a 7z file
/// #Example
/// ```no_run
/// sevenz_rust::decompress("sample.7z", "sample").expect("complete");
///
/// ```
///
pub fn decompress(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), Error> {
    let mut seven = SevenZReader::open(src.as_ref(), &[])?;
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
            std::io::copy(reader, &mut file).map_err(Error::io)?;
        }
        Ok(true)
    })?;

    Ok(())
}
