use core::{
    fmt::{Binary, Debug, Display, LowerExp, LowerHex, Octal, Pointer, UpperExp, UpperHex},
    ops::Deref,
};
use std::io::Write;

use derive_more::{AsMut, AsRef, From};
use serde::{Deserialize, Serialize};
use serde_hex::{Error as SerdeHexError, SerHex, Strict, StrictPfx};
use ssz::SszEncode;
use static_assertions::assert_not_impl_any;
use zeroize::Zeroize;

use crate::consts::SECRET_KEY_SIZE;

#[derive(Default, From, AsRef, AsMut, Zeroize, Deserialize)]
#[as_ref(forward)]
#[zeroize(drop)]
pub struct SecretKeyBytes {
    #[serde(with = "SerHex::<StrictPfx>")]
    pub(crate) bytes: [u8; SECRET_KEY_SIZE],
}

// Prevent `SecretKeyBytes` from implementing some traits to avoid leaking secret keys.
assert_not_impl_any! {
    SecretKeyBytes:

    Clone,
    Copy,
    ToOwned,
    Deref,

    Debug,
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

impl SerHex<Strict> for SecretKeyBytes {
    type Error = SerdeHexError;

    fn into_hex_raw<D: Write>(&self, _destination: D) -> Result<(), Self::Error> {
        unimplemented!("unimplemented to avoid leaking secret keys")
    }

    fn from_hex_raw<S: AsRef<[u8]>>(source: S) -> Result<Self, Self::Error> {
        let bytes = SerHex::<Strict>::from_hex_raw(source)?;
        Ok(Self { bytes })
    }
}
