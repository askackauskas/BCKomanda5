use thiserror::Error;

// The `unused_extern_crates` lint checks every crate in a package separately.
// See <https://github.com/rust-lang/rust/issues/57274>.
#[cfg(test)]
use ssz_new_derive as _;

pub use utils::{encode_items_from_parts, Decoder};

pub mod types;

mod decode;
mod encode;
mod utils;

pub const BYTES_PER_LENGTH_OFFSET: usize = 4;

pub trait SszEncode {
    fn as_ssz_bytes(&self) -> Vec<u8>;

    fn is_ssz_fixed_len() -> bool;
}

pub trait SszDecode: Sized {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError>;

    fn is_ssz_fixed_len() -> bool;

    #[must_use]
    fn ssz_fixed_len() -> usize {
        BYTES_PER_LENGTH_OFFSET
    }
}

#[derive(Debug, Error)]
#[error("{self:?}")]
pub enum SszDecodeError {
    InvalidByteLength { len: usize, expected: usize },
    InvalidLengthPrefix { len: usize, expected: usize },
    OutOfBoundsByte { i: usize },
    BytesInvalid(String),
}
