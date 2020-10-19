use core::convert::TryFrom;

use bls_eth_rust::Signature as RawSignature;

use crate::{consts::SIGNATURE_SIZE, Error, PublicKey, SignatureBytes};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Signature(pub(crate) RawSignature);

impl From<Signature> for SignatureBytes {
    fn from(signature: Signature) -> Self {
        Self::from_slice(signature.0.as_bytes().as_slice())
    }
}

impl TryFrom<SignatureBytes> for Signature {
    type Error = Error;

    fn try_from(bytes: SignatureBytes) -> Result<Self, Self::Error> {
        if bytes.0.iter().all(|byte| *byte == 0) {
            let mut empty_bytes = [0; SIGNATURE_SIZE];

            // See <https://tools.ietf.org/html/draft-boneh-bls-signature-00#section-2.6.1>.
            //
            // > the first bit indicates whether E is the point at infinity
            //
            // > the second bit of s_1 indicates whether the point is (x,y) or (x,-y)
            empty_bytes[0] = 0b1100_0000;

            return Ok(Self(
                RawSignature::from_serialized(&empty_bytes)
                    .expect("deserializing empty Signature should not fail"),
            ));
        }

        let signature = RawSignature::from_serialized(&bytes.0).map_err(Error)?;

        Ok(Self(signature))
    }
}

impl Signature {
    #[must_use]
    pub fn verify(self, public_key: PublicKey, message: impl AsRef<[u8]>) -> bool {
        // `RawSignature::verify` accepts a `*const PublicKey` but is not marked unsafe.
        // This is most likely an oversight.
        self.0.verify(&public_key.0, message.as_ref())
    }

    pub fn aggregate_in_place(&mut self, signature: Self) {
        // `RawSignature::add_assign` accepts a `*const Signature` but is not marked unsafe.
        // This is most likely an oversight.
        self.0.add_assign(&signature.0);
    }
}
