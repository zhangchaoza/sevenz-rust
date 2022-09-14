use std::io::{BufReader, ErrorKind, Read};

use crate::folder::Coder;

pub struct LzmaReader<R: Read> {
    reader: BufReader<R>,
    props_byte: u8,
    dict_size: u32,
    options: lzma_rs::decompress::Options,
    cache: Vec<u8>,
    cache_start: usize,
}

impl<R: Read> Read for LzmaReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            return Ok(0);
        }
        let start = self.cache_start;
        let mut cache = std::mem::replace(&mut self.cache, Default::default());
        if start > 0 && start < cache.len() {
            let cache_end = (start + buf.len()).min(cache.len());
            let data = &cache[self.cache_start..cache_end];
            let len = cache_end - self.cache_start;
            self.cache_start = cache_end;
            buf[..len].copy_from_slice(data);
            self.cache = cache;
            return Ok(len);
        } else if start > 0 && start == cache.len() {
            cache.clear();
            self.cache_start = 0;
        }
        let options = self.options;
        let mut header = [0u8; 5];

        header[0] = self.props_byte;
        header[1..].copy_from_slice(&self.dict_size.to_le_bytes());

        let mut reader = header.as_slice().chain(&mut self.reader);

        let result = match lzma_rs::lzma_decompress_with_options(&mut reader, &mut cache, &options)
        {
            Ok(_) => {
                //
                let len = cache.len();
                let cache_start = len.min(buf.len());
                buf[..cache_start].copy_from_slice(&cache[0..cache_start]);
                self.cache_start = cache_start;
                Ok(cache_start)
            }
            Err(e) => match e {
                lzma_rs::error::Error::IoError(e) => Err(e),
                lzma_rs::error::Error::HeaderTooShort(e) => Err(e),
                _ => Err(std::io::Error::new(ErrorKind::Other, e)),
            },
        };
        self.cache = cache;
        result
    }
}

impl<R: Read> LzmaReader<R> {
    pub fn new(inner: R, coder: &Coder, uncompressed_len: usize, memlimit: usize) -> Self {
        let props_byte = coder.properties[0];
        let dict_size = get_dic_size(coder);
        Self {
            reader: BufReader::new(inner),
            options: lzma_rs::decompress::Options {
                unpacked_size: lzma_rs::decompress::UnpackedSize::UseProvided(Some(
                    uncompressed_len as u64,
                )),
                memlimit: Some(memlimit),
                allow_incomplete: true,
            },
            props_byte,
            dict_size,
            cache: Vec::with_capacity(8192),
            cache_start: 0,
        }
    }
}
#[inline]
fn get_dic_size(coder: &Coder) -> u32 {
    let size = &coder.properties[1..];
    let size = [size[0], size[1], size[2], size[3]];
    u32::from_le_bytes(size)
}

