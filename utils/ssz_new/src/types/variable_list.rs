use ssz_types::VariableList;
use typenum::Unsigned;

use crate::{utils, SszDecode, SszDecodeError, SszEncode};

impl<T: SszEncode + Clone, N: Unsigned> SszEncode for VariableList<T, N> {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        let mut fixed_parts = Vec::with_capacity(self.len());
        for element in self {
            fixed_parts.push(if T::is_ssz_fixed_len() {
                Some(element.as_ssz_bytes())
            } else {
                None
            });
        }

        let mut variable_parts = Vec::with_capacity(self.len());
        for element in self {
            variable_parts.push(if T::is_ssz_fixed_len() {
                vec![]
            } else {
                element.as_ssz_bytes()
            });
        }

        utils::encode_items_from_parts(&fixed_parts, &variable_parts)
    }

    fn is_ssz_fixed_len() -> bool {
        false
    }
}

impl<T: SszDecode, N: Unsigned> SszDecode for VariableList<T, N> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        let fixed_len = T::ssz_fixed_len();

        if bytes.is_empty() {
            Ok(Self::empty())
        } else if !T::is_ssz_fixed_len() {
            utils::decode_variable_sized_items(bytes).map(Self::from)
        } else if bytes.len() % fixed_len == 0 {
            let mut items = Vec::with_capacity(bytes.len() / fixed_len);
            for chunk in bytes.chunks(fixed_len) {
                items.push(T::from_ssz_bytes(chunk)?);
            }
            Self::new(items).map_err(|e| {
                SszDecodeError::BytesInvalid(format!("Failed while creating VariableList: {:?}", e))
            })
        } else {
            Err(SszDecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: bytes.len() / fixed_len + 1,
            })
        }
    }

    fn is_ssz_fixed_len() -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use typenum::{U1, U1024, U20, U3, U4};

    use super::*;

    #[test]
    fn encode() {
        let vec = <VariableList<u16, U4>>::new(vec![1, 2, 3, 4]).expect("Test");
        assert_eq!(vec.as_ssz_bytes(), vec![1, 0, 2, 0, 3, 0, 4, 0]);

        let vec = <VariableList<u16, U20>>::new(vec![1, 2]).expect("Test");
        assert_eq!(vec.as_ssz_bytes(), vec![1, 0, 2, 0]);
    }

    #[test]
    fn decode() {
        let list = <VariableList<u16, U3>>::from_ssz_bytes(&[1, 0, 2, 0, 3, 0]).expect("Test");
        assert_eq!(list.to_vec(), vec![1_u16, 2_u16, 3_u16]);

        let list = <VariableList<u16, U1024>>::from_ssz_bytes(&[1, 0, 2, 0, 3, 0]).expect("Test");
        assert_eq!(list.to_vec(), vec![1_u16, 2_u16, 3_u16]);

        assert!(<VariableList<u8, U1>>::from_ssz_bytes(&[1, 2, 3]).is_err())
    }
}
