use std::io::Read;

use crate::{
    archive::SevenZMethod, error::Error, folder::Coder, lzma2_coder::Lzma2Reader,
    lzma_coder::LzmaReader,
};

pub enum Decoder<R: Read> {
    COPY(R),
    LZMA(LzmaReader<R>),
    LZMA2(Lzma2Reader<R>),
}

impl<R: Read> Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Decoder::COPY(r) => r.read(buf),
            Decoder::LZMA(r) => r.read(buf),
            Decoder::LZMA2(r) => r.read(buf),
        }
    }
}

pub fn add_decoder<I: Read>(
    input: I,
    uncompressed_len: usize,
    coder: &Coder,
    _password: &[u8],
    max_mem_limit_kb: usize,
) -> Result<Decoder<I>, Error> {
    let method = SevenZMethod::by_id(coder.decompression_method_id());
    let method = if let Some(m) = method {
        m
    } else {
        return Err(Error::UnsupportedCompressionMethod(format!(
            "{:?}",
            coder.decompression_method_id()
        )));
    };

    match method.id() {
        SevenZMethod::ID_COPY => Ok(Decoder::COPY(input)),
        SevenZMethod::ID_LZMA => {
            let lz = LzmaReader::new(input, coder, uncompressed_len, max_mem_limit_kb);
            Ok(Decoder::LZMA(lz))
        }
        SevenZMethod::ID_LZMA2 => {
            let lz = Lzma2Reader::new(input, coder)?;
            Ok(Decoder::LZMA2(lz))
        }
        _ => {
            return Err(Error::UnsupportedCompressionMethod(
                method.name().to_string(),
            ));
        }
    }
}
