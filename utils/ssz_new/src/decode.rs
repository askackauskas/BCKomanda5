use std::sync::Arc;

use ethereum_types::{H256, U256};

use crate::{SszDecode, SszDecodeError};

macro_rules! decode_for_uintn {
    ( $(($type_ident: ty, $size_in_bits: expr)),* ) => { $(
        impl SszDecode for $type_ident {
            fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
                if bytes.len() == Self::ssz_fixed_len() {
                    let mut arr = [0; $size_in_bits / 8];
                    arr.clone_from_slice(bytes);
                    Ok(Self::from_le_bytes(arr))
                } else {
                    Err(SszDecodeError::InvalidByteLength {
                        len: bytes.len(),
                        expected: Self::ssz_fixed_len(),
                    })
                }
            }

            fn is_ssz_fixed_len() -> bool {
                true
            }

            fn ssz_fixed_len() -> usize {
                $size_in_bits / 8
            }
        }
    )* };
}

decode_for_uintn!((u8, 8), (u16, 16), (u64, 64));

macro_rules! decode_for_u8_array {
    ($size: expr) => {
        impl SszDecode for [u8; $size] {
            fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
                if bytes.len() == Self::ssz_fixed_len() {
                    let mut array: [u8; $size] = [0; $size];
                    array.copy_from_slice(&bytes[..]);

                    Ok(array)
                } else {
                    Err(SszDecodeError::InvalidByteLength {
                        len: bytes.len(),
                        expected: Self::ssz_fixed_len(),
                    })
                }
            }

            fn is_ssz_fixed_len() -> bool {
                true
            }

            fn ssz_fixed_len() -> usize {
                $size
            }
        }
    };
}

decode_for_u8_array!(4);
decode_for_u8_array!(16);
decode_for_u8_array!(24);
decode_for_u8_array!(32);
decode_for_u8_array!(48);
decode_for_u8_array!(96);

// False positive in the `format!` macro.
#[allow(clippy::use_self)]
impl SszDecode for bool {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        if bytes.len() == Self::ssz_fixed_len() {
            match bytes[0] {
                0 => Ok(false),
                1 => Ok(true),
                _ => Err(SszDecodeError::BytesInvalid(format!(
                    "Cannot deserialize bool from {}",
                    bytes[0]
                ))),
            }
        } else {
            Err(SszDecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: Self::ssz_fixed_len(),
            })
        }
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        1
    }
}

impl<T: SszDecode> SszDecode for Arc<T> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        let inner = T::from_ssz_bytes(bytes)?;
        Ok(Self::new(inner))
    }

    fn is_ssz_fixed_len() -> bool {
        T::is_ssz_fixed_len()
    }

    fn ssz_fixed_len() -> usize {
        T::ssz_fixed_len()
    }
}

impl SszDecode for H256 {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        let len = bytes.len();
        let expected = Self::ssz_fixed_len();

        if len == expected {
            Ok(Self::from_slice(bytes))
        } else {
            Err(SszDecodeError::InvalidByteLength { len, expected })
        }
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }
}

impl SszDecode for U256 {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        let len = bytes.len();
        let expected = Self::ssz_fixed_len();

        if len == expected {
            Ok(Self::from_little_endian(bytes))
        } else {
            Err(SszDecodeError::InvalidByteLength { len, expected })
        }
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8() {
        assert_eq!(u8::from_ssz_bytes(&[0b0000_0000]).expect("Test"), 0);
        assert_eq!(u8::from_ssz_bytes(&[0b1111_1111]).expect("Test"), u8::MAX);
        assert_eq!(u8::from_ssz_bytes(&[0b0000_0001]).expect("Test"), 1);
        assert_eq!(u8::from_ssz_bytes(&[0b1000_0000]).expect("Test"), 128);

        assert!(u8::from_ssz_bytes(&[]).is_err());
        assert!(u8::from_ssz_bytes(&[0; 2]).is_err());

        assert_eq!(u8::ssz_fixed_len(), 1);
    }

    #[test]
    fn u16() {
        assert_eq!(
            u16::from_ssz_bytes(&[0b0000_0000, 0b0000_0000]).expect("Test"),
            0
        );
        assert_eq!(
            u16::from_ssz_bytes(&[0b0000_0001, 0b0000_0000]).expect("Test"),
            1
        );
        assert_eq!(
            u16::from_ssz_bytes(&[0b1000_0000, 0b0000_0000]).expect("Test"),
            128
        );
        assert_eq!(
            u16::from_ssz_bytes(&[0b1111_1111, 0b1111_1111]).expect("Test"),
            u16::MAX
        );
        assert_eq!(
            u16::from_ssz_bytes(&[0b0000_0000, 0b1000_0000]).expect("Test"),
            0x8000
        );

        assert!(u16::from_ssz_bytes(&[]).is_err());
        assert!(u16::from_ssz_bytes(&[0; 1]).is_err());
        assert!(u16::from_ssz_bytes(&[0; 3]).is_err());

        assert_eq!(u16::ssz_fixed_len(), 2);
    }

    #[test]
    fn u64() {
        assert_eq!(u64::from_ssz_bytes(&[0b0000_0000; 8]).expect("Test"), 0);
        assert_eq!(
            u64::from_ssz_bytes(&[0b1111_1111; 8]).expect("Test"),
            u64::MAX
        );
        assert_eq!(
            u64::from_ssz_bytes(&[
                0b0000_0001,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ])
            .expect("Test"),
            1
        );
        assert_eq!(
            u64::from_ssz_bytes(&[
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ])
            .expect("Test"),
            128
        );
        assert_eq!(
            u64::from_ssz_bytes(&[
                0b0000_0000,
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ])
            .expect("Test"),
            0x8000
        );
        assert_eq!(
            u64::from_ssz_bytes(&[
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ])
            .expect("Test"),
            0x8000_0000
        );
        assert_eq!(
            u64::from_ssz_bytes(&[
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b1000_0000
            ])
            .expect("Test"),
            0x8000_0000_0000_0000
        );

        assert!(u64::from_ssz_bytes(&[]).is_err());
        assert!(u64::from_ssz_bytes(&[0; 2]).is_err());
        assert!(u64::from_ssz_bytes(&[0; 9]).is_err());

        assert_eq!(u64::ssz_fixed_len(), 8);
    }

    #[test]
    fn u8_array() {
        assert_eq!(<[u8; 4]>::from_ssz_bytes(&[0; 4]).expect("Test"), [0; 4]);
        assert_eq!(<[u8; 32]>::from_ssz_bytes(&[0; 32]).expect("Test"), [0; 32]);
        assert_eq!(
            <[u8; 4]>::from_ssz_bytes(&[u8::MAX; 4]).expect("Test"),
            [u8::MAX; 4]
        );
        assert_eq!(
            <[u8; 32]>::from_ssz_bytes(&[u8::MAX; 32]).expect("Test"),
            [u8::MAX; 32]
        );

        assert!(<[u8; 4]>::from_ssz_bytes(&[0; 5]).is_err());
        assert!(<[u8; 32]>::from_ssz_bytes(&[0; 34]).is_err());

        assert_eq!(<[u8; 4]>::ssz_fixed_len(), 4);
        assert_eq!(<[u8; 32]>::ssz_fixed_len(), 32);

        assert!(<[u8; 4]>::is_ssz_fixed_len());
        assert!(<[u8; 32]>::is_ssz_fixed_len());
    }

    #[test]
    fn bool() {
        assert_eq!(bool::from_ssz_bytes(&[0_u8]).expect("Test"), false);
        assert_eq!(bool::from_ssz_bytes(&[1_u8]).expect("Test"), true);

        assert!(bool::from_ssz_bytes(&[2_u8]).is_err());
        assert!(bool::from_ssz_bytes(&[0_u8, 0_u8]).is_err());

        assert!(bool::is_ssz_fixed_len());
        assert_eq!(bool::ssz_fixed_len(), 1);
    }

    #[test]
    fn h256() {
        assert_eq!(H256::from_ssz_bytes(&[0; 32]).expect("Test"), H256::zero());

        assert!(H256::from_ssz_bytes(&[0; 31]).is_err());
        assert!(H256::from_ssz_bytes(&[0; 33]).is_err());

        assert!(H256::is_ssz_fixed_len());
        assert_eq!(H256::ssz_fixed_len(), 32)
    }

    #[test]
    fn u256() {
        assert_eq!(
            U256::from_ssz_bytes(&[0; 32]).expect("Test"),
            U256::from_dec_str("0").expect("Test")
        );

        assert!(U256::from_ssz_bytes(&[0; 31]).is_err());
        assert!(U256::from_ssz_bytes(&[0; 33]).is_err());

        assert!(U256::is_ssz_fixed_len());
        assert_eq!(U256::ssz_fixed_len(), 32)
    }
}
