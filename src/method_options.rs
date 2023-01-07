use std::fmt::Debug;

use crate::lzma::LZMA2Options;
#[derive(Debug, Clone)]
pub enum MethodOptions {
    Num(u32),
    LZMA2(LZMA2Options),
}

impl From<u32> for MethodOptions {
    fn from(n: u32) -> Self {
        Self::Num(n)
    }
}

impl From<LZMA2Options> for MethodOptions {
    fn from(o: LZMA2Options) -> Self {
        Self::LZMA2(o)
    }
}

impl MethodOptions {
    pub fn get_lzma2_dict_size(&self) -> u32 {
        match self {
            MethodOptions::Num(n) => *n,
            MethodOptions::LZMA2(o) => o.dict_size,
        }
    }
}
