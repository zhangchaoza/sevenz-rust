use lzma_rust::LZMA2Options;

#[cfg(feature = "aes256")]
use crate::{aes256sha256::AesEncoderOptions, Password};
use crate::{SevenZMethod, SevenZMethodConfiguration};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum MethodOptions {
    Num(u32),
    #[cfg(feature = "compress")]
    LZMA2(crate::lzma::LZMA2Options),
    #[cfg(feature = "aes256")]
    Aes(AesEncoderOptions),
}

#[cfg(feature = "aes256")]
impl From<AesEncoderOptions> for MethodOptions {
    fn from(value: AesEncoderOptions) -> Self {
        Self::Aes(value)
    }
}
#[cfg(feature = "aes256")]
impl From<AesEncoderOptions> for SevenZMethodConfiguration {
    fn from(value: AesEncoderOptions) -> Self {
        Self::new(SevenZMethod::AES256SHA256).with_options(MethodOptions::Aes(value))
    }
}

impl From<LZMA2Options> for SevenZMethodConfiguration {
    fn from(value: LZMA2Options) -> Self {
        Self::new(SevenZMethod::LZMA2).with_options(MethodOptions::LZMA2(value))
    }
}

impl From<u32> for MethodOptions {
    fn from(n: u32) -> Self {
        Self::Num(n)
    }
}

#[cfg(feature = "compress")]
impl From<crate::lzma::LZMA2Options> for MethodOptions {
    fn from(o: crate::lzma::LZMA2Options) -> Self {
        Self::LZMA2(o)
    }
}

impl MethodOptions {
    pub fn get_lzma2_dict_size(&self) -> u32 {
        match self {
            MethodOptions::Num(n) => *n,
            #[cfg(feature = "compress")]
            MethodOptions::LZMA2(o) => o.dict_size,
            _ => 0,
        }
    }
}
