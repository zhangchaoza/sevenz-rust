use std::io::{BufReader, ErrorKind, Read};

use crate::{folder::Coder, Error};

pub struct Lzma2Reader<R: Read> {
    reader: BufReader<R>,
    cache: Vec<u8>,
    cache_start: usize,
}

impl<R: Read> Read for Lzma2Reader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if buf.len() == 0 {
            println!("lzma2 decode buf len is 0");
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
        let mut reader =&mut self.reader;
        let result = match lzma_rs::lzma2_decompress(&mut reader, &mut cache) {
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

impl<R: Read> Lzma2Reader<R> {
    pub fn new(
        inner: R,
        coder: &Coder,
    ) -> Result<Self, Error> {
        let _dict_size = get_dic_size(coder)?;

        Ok(Self {
            reader: BufReader::new(inner),
            cache: Vec::with_capacity(8192),
            cache_start: 0,
        })
    }
}
#[inline]
fn get_dic_size(coder: &Coder) -> Result<u32, Error> {
    if coder.properties.len() < 1 {
        return Err(Error::other("LZMA2 properties too short"));
    }
    let dict_size_bits = 0xff & coder.properties[0] as u32;
    if (dict_size_bits & (!0x3f)) != 0 {
        return Err(Error::other("Unsupported LZMA2 property bits"));
    }
    if dict_size_bits > 40 {
        return Err(Error::other("Dictionary larger than 4GiB maximum size"));
    }
    if dict_size_bits == 40 {
        return Ok(0xFFFFffff);
    }
    let size = (2 | (dict_size_bits & 0x1)) << (dict_size_bits / 2 + 11);
    Ok(size)
}
