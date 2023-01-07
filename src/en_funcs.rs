//! 7z Compressor helper functions
//! 

use std::{
    fs::File,
    io::{Seek, Write},
    path::Path,
};

use crate::*;

/// hepler function to compress `src` path to `dest` writer
pub fn compress(src: impl AsRef<Path>, dest: impl Write + Seek) -> Result<(), Error> {
    let mut z = SevenZWriter::new(dest)?;
    let parent = if src.as_ref().is_dir() {
        src.as_ref()
    } else {
        src.as_ref().parent().unwrap_or(src.as_ref())
    };
    compress_path(src.as_ref(), parent, &mut z)?;
    z.finish().map_err(Error::io)
}

/// hepler function to compress `src` path to `dest` path
pub fn compress_to_path(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> Result<(), Error> {
    if let Some(p) = dest.as_ref().parent() {
        if !p.exists() {
            std::fs::create_dir_all(p)
                .map_err(|e| Error::io_msg(e, format!("Create dir failed:{:?}", dest.as_ref())))?;
        }
    }
    compress(
        src,
        File::create(dest.as_ref())
            .map_err(|e| Error::file_open(e, dest.as_ref().to_string_lossy().to_string()))?,
    )
}

fn compress_path<W: Write + Seek, P: AsRef<Path>>(
    src: P,
    root: &Path,
    z: &mut SevenZWriter<W>,
) -> Result<(), Error> {
    let entry_name = src
        .as_ref()
        .strip_prefix(root)
        .map_err(|e| Error::other(e.to_string()))?
        .to_string_lossy()
        .to_string();
    let entry = SevenZWriter::<W>::create_archive_entry(src.as_ref(), entry_name);
    let path = src.as_ref();
    if path.is_dir() {
        for dir in path
            .read_dir()
            .map_err(|e| Error::io_msg(e, "error read dir"))?
        {
            let dir = dir.map_err(Error::io)?;
            let ftype = dir.file_type().map_err(Error::io)?;
            if ftype.is_dir() || ftype.is_file() {
                compress_path(dir.path(), root, z)?;
            }
        }
    } else {
        z.push_archive_entry(
            entry,
            Some(
                File::open(path)
                    .map_err(|e| Error::file_open(e, path.to_string_lossy().to_string()))?,
            ),
        )?;
    }
    Ok(())
}
