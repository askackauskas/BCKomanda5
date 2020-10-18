use core::convert::TryInto as _;

use fixed_hash::construct_fixed_hash;
use impl_serde::impl_fixed_hash_serde;
use ssz::{SszDecode, SszDecodeError, SszEncode};
use tree_hash::{Hash256, MerkleHasher, TreeHash, TreeHashType, BYTES_PER_CHUNK};

use crate::{consts::SIGNATURE_SIZE, Error, Signature};

construct_fixed_hash! {
    pub struct SignatureBytes(SIGNATURE_SIZE);
}

impl_fixed_hash_serde!(SignatureBytes, SIGNATURE_SIZE);

impl SszEncode for SignatureBytes {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.0.as_ssz_bytes()
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

impl SszDecode for SignatureBytes {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        <[u8; SIGNATURE_SIZE]>::from_ssz_bytes(bytes).map(Self)
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        SIGNATURE_SIZE
    }
}

impl TreeHash for SignatureBytes {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> Vec<u8> {
        unreachable!("signatures are not a basic type and thus should never be packed")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("signatures are not a basic type and thus should never be packed")
    }

    fn tree_hash_root(&self) -> Hash256 {
        let num_leaves = (SIGNATURE_SIZE + BYTES_PER_CHUNK - 1) / BYTES_PER_CHUNK;
        let mut hasher = MerkleHasher::with_leaves(num_leaves);
        hasher
            .write(&self.0)
            .expect("the number of leaves as calculated above should be correct");
        hasher
            .finish()
            .expect("the number of leaves as calculated above should be correct")
    }
}

impl SignatureBytes {
    // REFACTOR(Sifrai Team): Move to `Signature` or inline once the validator is cleaned up.
    pub fn aggregate_in_place(&mut self, other: Self) -> Result<(), Error> {
        *self = self.aggregate(other)?;
        Ok(())
    }

    // REFACTOR(Sifrai Team): Move to `Signature` or inline once the validator is cleaned up.
    fn aggregate(self, other: Self) -> Result<Self, Error> {
        let mut aggregate_signature = Signature::default();
        aggregate_signature.aggregate_in_place(self.try_into()?);
        aggregate_signature.aggregate_in_place(other.try_into()?);
        Ok(aggregate_signature.into())
    }
}
