use std::sync::Arc;

use ethereum_types::{H256, U256};

use crate::SszEncode;

macro_rules! encode_for_uintn {
    ( $(($type_ident: ty, $size_in_bits: expr)),* ) => { $(
        impl SszEncode for $type_ident {
            fn as_ssz_bytes(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }

            fn is_ssz_fixed_len() -> bool {
                true
            }
        }
    )* };
}

encode_for_uintn!((u8, 8), (u16, 16), (u64, 64));

macro_rules! encode_for_u8_array {
    ($size: expr) => {
        impl SszEncode for [u8; $size] {
            fn as_ssz_bytes(&self) -> Vec<u8> {
                self.to_vec()
            }

            fn is_ssz_fixed_len() -> bool {
                true
            }
        }
    };
}

encode_for_u8_array!(4);
encode_for_u8_array!(48);
encode_for_u8_array!(96);

impl SszEncode for bool {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        let byte = if *self { 0b0000_0001 } else { 0b0000_0000 };
        vec![byte]
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

impl<T: SszEncode> SszEncode for Arc<T> {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        T::as_ssz_bytes(self)
    }

    fn is_ssz_fixed_len() -> bool {
        T::is_ssz_fixed_len()
    }
}

impl SszEncode for H256 {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

impl SszEncode for U256 {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        let mut result = vec![0; 32];
        self.to_little_endian(&mut result);
        result
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn u8() {
        assert_eq!(0_u8.as_ssz_bytes(), vec![0b0000_0000]);
        assert_eq!(u8::MAX.as_ssz_bytes(), vec![0b1111_1111]);
        assert_eq!(1_u8.as_ssz_bytes(), vec![0b0000_0001]);
        assert_eq!(128_u8.as_ssz_bytes(), vec![0b1000_0000]);

        assert!(u8::is_ssz_fixed_len());
    }

    #[test]
    fn u16() {
        assert_eq!(0_u16.as_ssz_bytes(), vec![0b0000_0000, 0b0000_0000]);
        assert_eq!(1_u16.as_ssz_bytes(), vec![0b0000_0001, 0b0000_0000]);
        assert_eq!(128_u16.as_ssz_bytes(), vec![0b1000_0000, 0b0000_0000]);
        assert_eq!(u16::MAX.as_ssz_bytes(), vec![0b1111_1111, 0b1111_1111]);
        assert_eq!(0x8000_u16.as_ssz_bytes(), vec![0b0000_0000, 0b1000_0000]);

        assert!(u16::is_ssz_fixed_len());
    }

    #[test]
    fn u64() {
        assert_eq!(0_u64.as_ssz_bytes(), vec![0b0000_0000; 8]);
        assert_eq!(u64::MAX.as_ssz_bytes(), vec![0b1111_1111; 8]);
        assert_eq!(
            1_u64.as_ssz_bytes(),
            vec![
                0b0000_0001,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ]
        );
        assert_eq!(
            128_u64.as_ssz_bytes(),
            vec![
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ]
        );
        assert_eq!(
            0x8000_u64.as_ssz_bytes(),
            vec![
                0b0000_0000,
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ]
        );
        assert_eq!(
            0x8000_0000_u64.as_ssz_bytes(),
            vec![
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b1000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000
            ]
        );
        assert_eq!(
            0x8000_0000_0000_0000_u64.as_ssz_bytes(),
            vec![
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b0000_0000,
                0b1000_0000
            ]
        );

        assert!(u64::is_ssz_fixed_len());
    }

    #[test]
    fn bool() {
        assert_eq!(true.as_ssz_bytes(), vec![0b0000_0001]);
        assert_eq!(false.as_ssz_bytes(), vec![0b0000_0000]);

        assert!(bool::is_ssz_fixed_len());
    }

    #[test]
    fn u8_array() {
        assert_eq!([1; 4].as_ssz_bytes(), vec![1; 4]);

        assert!(<[u8; 4]>::is_ssz_fixed_len());
    }

    #[test]
    fn h256() {
        assert_eq!(H256::zero().as_ssz_bytes(), vec![0; 32]);

        assert!(H256::is_ssz_fixed_len());
    }

    #[test]
    fn u256() {
        let u = U256::from_dec_str("0").expect("Test");
        assert_eq!(u.as_ssz_bytes(), vec![0; 32]);

        assert!(U256::is_ssz_fixed_len());
    }
}
