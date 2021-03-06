use crate::{SszDecode, SszDecodeError, BYTES_PER_LENGTH_OFFSET};

#[must_use]
pub fn encode_offset(offset: usize) -> Vec<u8> {
    offset.to_le_bytes()[..BYTES_PER_LENGTH_OFFSET].to_vec()
}

#[must_use]
pub fn encode_items_from_parts(
    fixed_parts: &[Option<Vec<u8>>],
    variable_parts: &[Vec<u8>],
) -> Vec<u8> {
    let item_count = fixed_parts.len();

    let fixed_length: usize = fixed_parts
        .iter()
        .map(|part| match part {
            Some(bytes) => bytes.len(),
            None => BYTES_PER_LENGTH_OFFSET,
        })
        .sum();

    let variable_lengths: Vec<usize> = variable_parts.iter().map(std::vec::Vec::len).collect();

    let mut variable_offsets = Vec::with_capacity(item_count);
    for i in 0..item_count {
        let variable_length_sum: usize = variable_lengths[..i].iter().sum();
        let offset = fixed_length + variable_length_sum;
        variable_offsets.push(encode_offset(offset));
    }

    let fixed_parts: Vec<&Vec<u8>> = fixed_parts
        .iter()
        .enumerate()
        .map(|(i, part)| match part {
            Some(bytes) => bytes,
            None => &variable_offsets[i],
        })
        .collect();

    let variable_lengths_sum: usize = variable_lengths.iter().sum();
    let total_bytes = fixed_length + variable_lengths_sum;
    let mut result = Vec::with_capacity(total_bytes);

    for part in fixed_parts {
        result.extend(part);
    }

    for part in variable_parts {
        result.extend(part);
    }

    result
}

pub fn decode_offset(bytes: &[u8]) -> Result<usize, SszDecodeError> {
    if bytes.len() == BYTES_PER_LENGTH_OFFSET {
        let mut arr = [0; BYTES_PER_LENGTH_OFFSET];
        arr.clone_from_slice(bytes);
        Ok(u32::from_le_bytes(arr) as usize)
    } else {
        Err(SszDecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: BYTES_PER_LENGTH_OFFSET,
        })
    }
}

pub fn decode_variable_sized_items<T: SszDecode>(bytes: &[u8]) -> Result<Vec<T>, SszDecodeError> {
    let first_offset_bytes = bytes.get(0..BYTES_PER_LENGTH_OFFSET);
    let first_offset = match first_offset_bytes {
        Some(bytes) => decode_offset(bytes),
        None => Err(SszDecodeError::InvalidByteLength {
            len: bytes.len(),
            expected: BYTES_PER_LENGTH_OFFSET,
        }),
    }?;

    let number_of_elements = first_offset / BYTES_PER_LENGTH_OFFSET;
    let mut result = Vec::with_capacity(number_of_elements);

    let mut previous_offset = first_offset;
    for i in 1..=number_of_elements {
        let next_offset = if i == number_of_elements {
            bytes.len()
        } else {
            match bytes.get(i * BYTES_PER_LENGTH_OFFSET..(i + 1) * BYTES_PER_LENGTH_OFFSET) {
                Some(bytes) => decode_offset(bytes),
                None => Err(SszDecodeError::InvalidByteLength {
                    len: bytes.len(),
                    expected: (i + 1) * BYTES_PER_LENGTH_OFFSET,
                }),
            }?
        };

        let element = match bytes.get(previous_offset..next_offset) {
            Some(bytes) => T::from_ssz_bytes(bytes),
            None => Err(SszDecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: next_offset,
            }),
        }?;

        result.push(element);
        previous_offset = next_offset;
    }
    Ok(result)
}

pub struct Decoder<'a> {
    bytes: &'a [u8],
    registration_offset: usize,
    fixed_part_offset: usize,
    offsets: Vec<usize>,
    current_offset_index: usize,
}

impl<'a> Decoder<'a> {
    #[must_use]
    pub const fn for_bytes(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            registration_offset: 0,
            fixed_part_offset: 0,
            offsets: vec![],
            current_offset_index: 0,
        }
    }

    pub fn next_type<T: SszDecode>(&mut self) -> Result<(), SszDecodeError> {
        if !T::is_ssz_fixed_len() {
            let offset = match self
                .bytes
                .get(self.registration_offset..self.registration_offset + BYTES_PER_LENGTH_OFFSET)
            {
                Some(bytes) => decode_offset(bytes),
                None => Err(SszDecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: self.registration_offset + BYTES_PER_LENGTH_OFFSET,
                }),
            }?;
            self.offsets.push(offset);
        }
        self.registration_offset += T::ssz_fixed_len();
        Ok(())
    }

    pub fn deserialize_next<T: SszDecode>(&mut self) -> Result<T, SszDecodeError> {
        let result = if T::is_ssz_fixed_len() {
            match self
                .bytes
                .get(self.fixed_part_offset..self.fixed_part_offset + T::ssz_fixed_len())
            {
                Some(bytes) => T::from_ssz_bytes(bytes),
                None => Err(SszDecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: self.fixed_part_offset + T::ssz_fixed_len(),
                }),
            }
        } else {
            let current_offset = match self.offsets.get(self.current_offset_index) {
                Some(offset) => Ok(*offset),
                None => Err(SszDecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: self.current_offset_index,
                }),
            }?;

            let next_offset = match self.offsets.get(self.current_offset_index + 1) {
                Some(offset) => *offset,
                None => self.bytes.len(),
            };

            match self.bytes.get(current_offset..next_offset) {
                Some(bytes) => T::from_ssz_bytes(bytes),
                None => Err(SszDecodeError::InvalidByteLength {
                    len: self.bytes.len(),
                    expected: next_offset,
                }),
            }
        };

        if result.is_ok() {
            if !T::is_ssz_fixed_len() {
                self.current_offset_index += 1;
            }
            self.fixed_part_offset += T::ssz_fixed_len();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_offset() {
        assert_eq!(encode_offset(0), vec![0; BYTES_PER_LENGTH_OFFSET]);
        assert_eq!(encode_offset(5), vec![5, 0, 0, 0]);
    }

    #[test]
    fn test_deserialize_offset() {
        assert_eq!(
            decode_offset(&[0; BYTES_PER_LENGTH_OFFSET]).expect("Test"),
            0
        );
        assert_eq!(decode_offset(&[5, 0, 0, 0]).expect("Test"), 5);
    }

    #[test]
    fn test_deserialize_offset_error() {
        assert!(decode_offset(&[0; BYTES_PER_LENGTH_OFFSET + 1]).is_err());
    }

    mod decoder {
        use ssz_types::VariableList;
        use typenum::U4;

        use super::*;

        #[test]
        fn only_fixed() {
            let mut decoder = Decoder::for_bytes(&[1, 2, 3, 4]);
            decoder.next_type::<u8>().expect("Test");
            decoder.next_type::<u8>().expect("Test");
            decoder.next_type::<u8>().expect("Test");
            decoder.next_type::<u8>().expect("Test");
            assert_eq!(decoder.deserialize_next::<u8>().expect("Test"), 1);
            assert_eq!(decoder.deserialize_next::<u8>().expect("Test"), 2);
            assert_eq!(decoder.deserialize_next::<u8>().expect("Test"), 3);
            assert_eq!(decoder.deserialize_next::<u8>().expect("Test"), 4);
        }

        #[test]
        fn single_variable_list() -> Result<(), SszDecodeError> {
            let mut decoder = Decoder::for_bytes(&[4, 0, 0, 0, 1, 2, 3, 4]);
            decoder.next_type::<VariableList<u8, U4>>()?;
            assert_eq!(
                decoder.deserialize_next::<VariableList<u8, U4>>()?,
                vec![1, 2, 3, 4].into(),
            );
            Ok(())
        }

        #[test]
        fn mixed() -> Result<(), SszDecodeError> {
            let mut decoder = Decoder::for_bytes(&[
                1, 13, 0, 0, 0, 255, 255, 255, 255, 16, 0, 0, 0, 3, 2, 3, 1, 0, 2, 0, 3, 0,
            ]);
            decoder.next_type::<bool>()?;
            decoder.next_type::<VariableList<u8, U4>>()?;
            decoder.next_type::<u16>()?;
            decoder.next_type::<u16>()?;
            decoder.next_type::<VariableList<u16, U4>>()?;
            assert_eq!(decoder.deserialize_next::<bool>()?, true);
            assert_eq!(
                decoder.deserialize_next::<VariableList<u8, U4>>()?,
                vec![3, 2, 3].into(),
            );
            assert_eq!(decoder.deserialize_next::<u16>()?, u16::MAX);
            assert_eq!(decoder.deserialize_next::<u16>()?, u16::MAX);
            assert_eq!(
                decoder.deserialize_next::<VariableList<u16, U4>>()?,
                vec![1, 2, 3].into(),
            );
            Ok(())
        }

        #[test]
        fn errors() -> Result<(), SszDecodeError> {
            let mut decoder = Decoder::for_bytes(&[1]);
            assert!(decoder.deserialize_next::<u8>().is_ok());
            assert!(decoder.deserialize_next::<u8>().is_err());

            let mut decoder = Decoder::for_bytes(&[1]);
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_err());

            let mut decoder = Decoder::for_bytes(&[8, 0, 0, 0, 255, 0, 0, 0]);
            decoder.next_type::<VariableList<u8, U4>>()?;
            decoder.next_type::<VariableList<u8, U4>>()?;
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_err());
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_err());

            let mut decoder = Decoder::for_bytes(&[12, 0, 0, 0, 12, 0, 0, 0, 255, 0, 0, 0]);
            decoder.next_type::<VariableList<u8, U4>>()?;
            decoder.next_type::<VariableList<u8, U4>>()?;
            decoder.next_type::<VariableList<u8, U4>>()?;
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_ok());
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_err());
            assert!(decoder.deserialize_next::<VariableList<u8, U4>>().is_err());

            Ok(())
        }
    }
}
