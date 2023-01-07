#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(feature = "aes256")]
mod aes256sha256;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub(crate) mod archive;
mod bcj;
#[cfg(not(target_arch = "wasm32"))]
mod de_funcs;
pub(crate) mod decoders;
mod delta;
#[cfg(feature = "compress")]
mod en_funcs;
mod encoders;
mod error;
pub(crate) mod folder;
mod lzma;
mod method_options;
mod password;
mod reader;
mod writer;
pub use archive::SevenZArchiveEntry;
#[cfg(not(target_arch = "wasm32"))]
pub use de_funcs::*;
#[cfg(feature = "compress")]
pub use en_funcs::*;
pub use error::Error;
pub use password::Password;
pub use reader::SevenZReader;
pub use writer::SevenZWriter;
