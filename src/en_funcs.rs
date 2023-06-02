//! 7z Compressor helper functions
//!

use std::{
    fs::{File, FileType},
    io::{Seek, Write},
    path::{Path, PathBuf},
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

#[cfg(feature = "aes256")]
pub fn compress_encypted(
    src: impl AsRef<Path>,
    dest: impl Write + Seek,
    password: Password,
) -> Result<(), Error> {
    let mut z = SevenZWriter::new(dest)?;
    if !password.is_empty() {
        z.set_content_methods(vec![
            aes256sha256::AesEncoderOptions::new(password).into(),
            SevenZMethod::LZMA2.into(),
        ]);
    }
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

#[cfg(feature = "aes256")]
pub fn compress_to_path_encrypted(
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    password: Password,
) -> Result<(), Error> {
    if let Some(p) = dest.as_ref().parent() {
        if !p.exists() {
            std::fs::create_dir_all(p)
                .map_err(|e| Error::io_msg(e, format!("Create dir failed:{:?}", dest.as_ref())))?;
        }
    }
    compress_encypted(
        src,
        File::create(dest.as_ref())
            .map_err(|e| Error::file_open(e, dest.as_ref().to_string_lossy().to_string()))?,
        password,
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
    let entry = SevenZArchiveEntry::from_path(src.as_ref(), entry_name);
    let path = src.as_ref();
    if path.is_dir() {
        // z.push_archive_entry::<&[u8]>(entry, None)?;
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

impl<W: Write + Seek> SevenZWriter<W> {
    /// [Solid compression](https://en.wikipedia.org/wiki/Solid_compression)
    /// compress all files in [path] into one block
    ///
    #[inline]
    pub fn push_source_path(
        &mut self,
        path: impl AsRef<Path>,
        filter: impl Fn(&Path) -> bool,
    ) -> Result<&mut Self, crate::Error> {
        let (entries, reader) = collect_entries_and_reader(&path, filter).map_err(|e| {
            crate::Error::io_msg(
                e,
                format!("Failed to collect entries from path:{:?}", path.as_ref()),
            )
        })?;
        self.push_archive_entries(entries, reader)
    }
}

fn collect_file_paths(
    src: impl AsRef<Path>,
    paths: &mut Vec<PathBuf>,
    filter: &dyn Fn(&Path) -> bool,
) -> std::io::Result<()> {
    let path = src.as_ref();
    if !filter(path) {
        return Ok(());
    }
    if path.is_dir() {
        for dir in path.read_dir()? {
            let dir = dir?;
            let ftype = dir.file_type()?;
            if ftype.is_file() || ftype.is_dir() {
                collect_file_paths(dir.path(), paths, filter)?;
            }
        }
    } else {
        paths.push(path.to_path_buf())
    }
    Ok(())
}

pub fn collect_entries_and_reader(
    src: impl AsRef<Path>,
    filter: impl Fn(&Path) -> bool,
) -> std::io::Result<(Vec<SevenZArchiveEntry>, SeqReader<SourceReader<File>>)> {
    let mut entries = Vec::new();
    let mut paths = Vec::new();
    collect_file_paths(&src, &mut paths, &filter)?;
    for ele in paths.iter() {
        let name = ele
            .strip_prefix(&src)
            .unwrap()
            .to_string_lossy()
            .to_string();

        entries.push(SevenZArchiveEntry::from_path(ele, name))
    }
    let files: Vec<SourceReader<_>> = paths
        .iter()
        .map(|p| File::open(p).unwrap().into())
        .collect();

    Ok((entries, SeqReader::new(files)))
}
