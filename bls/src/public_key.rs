use core::convert::TryFrom;

use bls_eth_rust::PublicKey as RawPublicKey;

use crate::{Error, PublicKeyBytes};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct PublicKey(pub(crate) RawPublicKey);

impl From<PublicKey> for PublicKeyBytes {
    fn from(public_key: PublicKey) -> Self {
        Self::from_slice(public_key.0.as_bytes().as_slice())
    }
}

impl TryFrom<PublicKeyBytes> for PublicKey {
    type Error = Error;

    fn try_from(bytes: PublicKeyBytes) -> Result<Self, Self::Error> {
        let public_key = RawPublicKey::from_serialized(&bytes.0).map_err(Error)?;
        Ok(Self(public_key))
    }
}

impl PublicKey {
    pub fn aggregate_in_place(&mut self, public_key: Self) {
        // `RawPublicKey::add_assign` accepts a `*const PublicKey` but is not marked unsafe.
        // This is most likely an oversight.
        self.0.add_assign(&public_key.0);
    }
}
