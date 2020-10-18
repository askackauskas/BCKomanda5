use std::io::Write;

use fixed_hash::construct_fixed_hash;
use impl_serde::impl_fixed_hash_serde;
use serde_hex::{Error as SerdeHexError, SerHex, Strict};
use ssz::{SszDecode, SszDecodeError, SszEncode};
use tree_hash::{Hash256, MerkleHasher, TreeHash, TreeHashType, BYTES_PER_CHUNK};

use crate::consts::PUBLIC_KEY_SIZE;

construct_fixed_hash! {
    pub struct PublicKeyBytes(PUBLIC_KEY_SIZE);
}

impl_fixed_hash_serde!(PublicKeyBytes, PUBLIC_KEY_SIZE);

impl SerHex<Strict> for PublicKeyBytes {
    type Error = SerdeHexError;

    fn into_hex_raw<D: Write>(&self, destination: D) -> Result<(), Self::Error> {
        SerHex::<Strict>::into_hex_raw(&self.0, destination)
    }

    fn from_hex_raw<S: AsRef<[u8]>>(source: S) -> Result<Self, Self::Error> {
        SerHex::<Strict>::from_hex_raw(source).map(Self)
    }
}

impl SszEncode for PublicKeyBytes {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        self.0.as_ssz_bytes()
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }
}

impl SszDecode for PublicKeyBytes {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, SszDecodeError> {
        <[u8; PUBLIC_KEY_SIZE]>::from_ssz_bytes(bytes).map(Self)
    }

    fn is_ssz_fixed_len() -> bool {
        true
    }

    fn ssz_fixed_len() -> usize {
        PUBLIC_KEY_SIZE
    }
}

impl TreeHash for PublicKeyBytes {
    fn tree_hash_type() -> TreeHashType {
        TreeHashType::Vector
    }

    fn tree_hash_packed_encoding(&self) -> Vec<u8> {
        unreachable!("public keys are not a basic type and thus should never be packed")
    }

    fn tree_hash_packing_factor() -> usize {
        unreachable!("public keys are not a basic type and thus should never be packed")
    }

    fn tree_hash_root(&self) -> Hash256 {
        let num_leaves = (PUBLIC_KEY_SIZE + BYTES_PER_CHUNK - 1) / BYTES_PER_CHUNK;
        let mut hasher = MerkleHasher::with_leaves(num_leaves);
        hasher
            .write(&self.0)
            .expect("the number of leaves as calculated above should be correct");
        hasher
            .finish()
            .expect("the number of leaves as calculated above should be correct")
    }
}
