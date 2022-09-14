use std::io::Read;

use crate::{
    archive::SevenZMethod, bcj::SimpleReader, delta::DeltaReader, error::Error, folder::Coder,
    lzma2_coder::Lzma2Reader, lzma_coder::LzmaReader,
};

pub enum Decoder<R: Read> {
    COPY(R),
    LZMA(LzmaReader<R>),
    LZMA2(Lzma2Reader<R>),
    BCJ(SimpleReader<R>),
    Delta(DeltaReader<R>),
}

impl<R: Read> Read for Decoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            Decoder::COPY(r) => r.read(buf),
            Decoder::LZMA(r) => r.read(buf),
            Decoder::LZMA2(r) => r.read(buf),
            Decoder::BCJ(r) => r.read(buf),
            Decoder::Delta(r) => r.read(buf),
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
    println!("method:{}", method.name());
    match method.id() {
        SevenZMethod::ID_COPY => Ok(Decoder::COPY(input)),
        SevenZMethod::ID_LZMA => {
            let lz = LzmaReader::new(input, coder, uncompressed_len, max_mem_limit_kb);
            Ok(Decoder::LZMA(lz))
        }
        SevenZMethod::ID_LZMA2 => {
            let lz = Lzma2Reader::new(input, coder, max_mem_limit_kb)?;
            Ok(Decoder::LZMA2(lz))
        }
        SevenZMethod::ID_BCJ_X86 => {
            let de = SimpleReader::new_x86(input);
            Ok(Decoder::BCJ(de))
        }
        SevenZMethod::ID_BCJ_ARM => {
            let de = SimpleReader::new_arm(input);
            Ok(Decoder::BCJ(de))
        }
        SevenZMethod::ID_BCJ_ARM_THUMB => {
            let de = SimpleReader::new_arm_thumb(input);
            Ok(Decoder::BCJ(de))
        }
        SevenZMethod::ID_BCJ_PPC => {
            let de = SimpleReader::new_ppc(input);
            Ok(Decoder::BCJ(de))
        }
        SevenZMethod::ID_BCJ_SPARC => {
            let de = SimpleReader::new_sparc(input);
            Ok(Decoder::BCJ(de))
        }
        SevenZMethod::ID_DELTA => {
            let d = if coder.properties.is_empty() {
                1
            } else {
                (coder.properties[0] & 0xff) + 1
            };
            let de = DeltaReader::new(input, d as usize);
            Ok(Decoder::Delta(de))
        }
        _ => {
            return Err(Error::UnsupportedCompressionMethod(
                method.name().to_string(),
            ));
        }
    }
}
