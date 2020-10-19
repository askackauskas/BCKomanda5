use core::{
    convert::{TryFrom, TryInto as _},
    fmt::{Binary, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
    hash::{Hash, Hasher},
    ops::Deref,
};
use std::borrow::ToOwned;

use bls_eth_rust::SecretKey as RawSecretKey;
use derive_more::DebugCustom;
use serde::{
    de::{Deserializer, Error as _},
    Deserialize, Serialize,
};
use ssz::SszEncode;
use static_assertions::assert_not_impl_any;
use zeroize::{DefaultIsZeroes, Zeroize};

use crate::{Error, PublicKey, SecretKeyBytes, Signature};

#[derive(Eq, PartialEq, DebugCustom, Zeroize)]
// Inspired by `DebugSecret` from the `secrecy` crate.
#[debug(fmt = "[REDACTED]")]
#[zeroize(drop)]
pub struct SecretKey(ZeroableRawSecretKey);

// Prevent `SecretKey` from implementing some traits to avoid leaking secret keys.
// `Secret` from the `secrecy` crate could be used as an alternative to this.
assert_not_impl_any! {
    SecretKey:

    Clone,
    Copy,
    ToOwned,
    Deref,

    Binary,
    Display,
    LowerExp,
    LowerHex,
    Octal,
    Pointer,
    UpperExp,
    UpperHex,

    Serialize,
    SszEncode,
}

impl TryFrom<SecretKeyBytes> for SecretKey {
    type Error = Error;

    fn try_from(secret_key_bytes: SecretKeyBytes) -> Result<Self, Self::Error> {
        let secret_key = RawSecretKey::from_serialized(&secret_key_bytes.bytes).map_err(Error)?;
        Ok(Self(ZeroableRawSecretKey(secret_key)))
    }
}

// `Hash` cannot be derived for `SecretKey` because `RawSecretKey` does not implement it.
// This impl may be incorrect. `bls_eth_rust` tests for equality using its internal representation
// rather than the serialized representation.
#[allow(clippy::derive_hash_xor_eq)]
impl Hash for SecretKey {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_raw().as_bytes().hash(hasher)
    }
}

impl<'deserializer> Deserialize<'deserializer> for SecretKey {
    fn deserialize<D: Deserializer<'deserializer>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = SecretKeyBytes::deserialize(deserializer)?;
        bytes.try_into().map_err(D::Error::custom)
    }
}

impl SecretKey {
    #[must_use]
    pub fn to_public_key(&self) -> PublicKey {
        PublicKey(self.as_raw().get_publickey())
    }

    #[must_use]
    pub fn sign(&self, message: impl AsRef<[u8]>) -> Signature {
        Signature(self.as_raw().sign(message.as_ref()))
    }

    const fn as_raw(&self) -> &RawSecretKey {
        &(self.0).0
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct ZeroableRawSecretKey(RawSecretKey);

impl DefaultIsZeroes for ZeroableRawSecretKey {}
