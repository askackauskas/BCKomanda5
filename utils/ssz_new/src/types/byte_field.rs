use ssz_types::{
    length::{Fixed, Variable},
    Bytefield,
};
use typenum::Unsigned;

use crate::{SszDecode, SszDecodeError, SszEncode};

impl<N: Unsigned + Clone> SszEncode for Bytefield<Variable<N>> {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }

    fn is_ssz_fixed_len() -> bool {
        false
    }
}

impl<N: Unsigned + Clone> SszDecode for Bytefield<Variable<N>> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        Self::from_bytes(bytes.to_vec()).map_err(|e| {
            SszDecodeError::BytesInvalid(format!("Failed while creating ByteList: {:?}", e))
        })
    }

    fn is_ssz_fixed_len() -> bool {
        false
    }
}

impl<N: Unsigned + Clone> SszEncode for Bytefield<Fixed<N>> {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.clone().into_bytes()
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

impl<N: Unsigned + Clone> SszDecode for Bytefield<Fixed<N>> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        Self::from_bytes(bytes.to_vec()).map_err(|e| {
            SszDecodeError::BytesInvalid(format!("Failed while creating ByteVector: {:?}", e))
        })
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        N::USIZE
    }
}
