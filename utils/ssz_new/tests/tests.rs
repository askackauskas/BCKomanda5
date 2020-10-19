use ethereum_types::U256;
use ssz_new_derive::{SszDecode, SszEncode};
use ssz_types::VariableList;
use typenum::U5;

// The `unused_extern_crates` lint checks every crate in a package separately.
// See <https://github.com/rust-lang/rust/issues/57274>.
#[cfg(test)]
use thiserror as _;

#[derive(SszEncode, SszDecode, PartialEq, Debug)]
struct Fixed {
    a: u16,
    b: bool,
}

#[derive(SszEncode, SszDecode, PartialEq, Debug)]
struct Variable {
    a: u16,
    b: VariableList<u8, U5>,
    c: bool,
}

#[derive(SszEncode, SszDecode, PartialEq, Debug)]
struct Nested {
    fixed: Fixed,
    variable: Variable,
}

#[derive(SszEncode, SszDecode, PartialEq, Debug)]
struct Skippable {
    stay_1: [u8; 4],

    #[ssz(skip_serializing)]
    #[ssz(skip_deserializing)]
    skip_1: u8,

    #[ssz(skip_serializing)]
    #[ssz(skip_deserializing)]
    skip_2: Vec<u8>,

    stay_2: VariableList<u8, U5>,
}

#[derive(SszEncode, SszDecode, PartialEq, Debug)]
struct NestedVariable {
    a: VariableList<U256, U5>,
    b: VariableList<U256, U5>,
}

mod serialize_derive {
    use ssz_new::SszEncode;

    use crate::{Fixed, Nested, Variable};

    #[test]
    fn is_fixed_size() {
        assert!(!<Nested as SszEncode>::is_ssz_fixed_len());
        assert!(!<Variable as SszEncode>::is_ssz_fixed_len());
        assert!(<Fixed as SszEncode>::is_ssz_fixed_len());
    }

    #[test]
    fn serialize_fixed_struct() {
        let fixed = Fixed { a: 22, b: true };

        assert_eq!(fixed.as_ssz_bytes(), vec![22, 0, 1])
    }

    #[test]
    fn serialize_variable_struct() {
        let variable = Variable {
            a: u16::MAX,
            b: vec![1, 2, 3, 4, 5].into(),
            c: false,
        };

        assert_eq!(
            variable.as_ssz_bytes(),
            vec![u8::MAX, u8::MAX, 7, 0, 0, 0, 0, 1, 2, 3, 4, 5]
        )
    }

    #[test]
    fn serialize_nested_struct() {
        let nested = Nested {
            fixed: Fixed { a: 5, b: false },
            variable: Variable {
                a: 80,
                b: vec![1, 2, 3, 4].into(),
                c: true,
            },
        };

        assert_eq!(
            nested.as_ssz_bytes(),
            vec![5, 0, 0, 7, 0, 0, 0, 80, 0, 7, 0, 0, 0, 1, 1, 2, 3, 4]
        );
    }
}

mod deserialize_derive {
    use ssz_new::{SszDecode as _, SszEncode as _};

    use crate::{Fixed, Nested, Skippable, Variable};

    #[test]
    fn deserialize_fixed_struct() {
        let fixed = Fixed { a: 22, b: true };

        assert_eq!(
            Fixed::from_ssz_bytes(&[22, 0, 1]).expect("bytes represent a valid Fixed struct"),
            fixed
        );
    }

    #[test]
    fn deserialize_variable_struct() {
        let variable = Variable {
            a: u16::MAX,
            b: vec![1, 2, 3, 4, 5].into(),
            c: false,
        };

        assert_eq!(
            Variable::from_ssz_bytes(&[u8::MAX, u8::MAX, 7, 0, 0, 0, 0, 1, 2, 3, 4, 5])
                .expect("bytes represent a valid Variable struct"),
            variable
        );
    }

    #[test]
    fn deserialize_nested_struct() {
        let nested = Nested {
            fixed: Fixed { a: 5, b: false },
            variable: Variable {
                a: 80,
                b: vec![1, 2, 3, 4].into(),
                c: true,
            },
        };

        assert_eq!(
            Nested::from_ssz_bytes(&[5, 0, 0, 7, 0, 0, 0, 80, 0, 7, 0, 0, 0, 1, 1, 2, 3, 4])
                .expect("bytes represent a valid Nested struct"),
            nested
        );
    }

    #[test]
    fn skip_fields() {
        let skippable = Skippable {
            stay_1: [1, 2, 3, 4],
            stay_2: vec![1, 2, 3, 4, 5].into(),
            skip_1: 42,
            skip_2: vec![6, 7, 8, 9, 10],
        };

        let serialized = skippable.as_ssz_bytes();
        assert_eq!(serialized, vec![1, 2, 3, 4, 8, 0, 0, 0, 1, 2, 3, 4, 5]);

        let skippable = Skippable::from_ssz_bytes(serialized.as_slice()).expect("Test");
        assert_eq!(skippable.skip_1, <u8>::default());
        assert_eq!(skippable.skip_2, <Vec<u8>>::default());
    }
}

mod round_trips {
    use ethereum_types::U256;
    use ssz_new::{SszDecode, SszEncode};

    use crate::NestedVariable;

    #[test]
    fn nested_variable() {
        let item = NestedVariable {
            a: vec![
                U256::from_dec_str("12345").expect("Test"),
                U256::from_dec_str("12345").expect("Test"),
                U256::from_dec_str("12345").expect("Test"),
                U256::from_dec_str("12345").expect("Test"),
            ]
            .into(),
            b: vec![U256::from_dec_str("12345").expect("Test")].into(),
        };

        assert_round_trip(&item);
        assert_eq!(
            NestedVariable::ssz_fixed_len(),
            ssz_new::BYTES_PER_LENGTH_OFFSET
        );
    }

    fn assert_round_trip<T: SszEncode + SszDecode + PartialEq + std::fmt::Debug>(t: &T) {
        assert_eq!(&T::from_ssz_bytes(&t.as_ssz_bytes()).expect("Test"), t);
    }
}
