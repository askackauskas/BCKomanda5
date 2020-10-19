use core::convert::TryInto as _;

use anyhow::Result;
use bls::{PublicKey, PublicKeyBytes, Signature, SignatureBytes};

pub fn bls_verify(
    pubkey: PublicKeyBytes,
    message: &[u8],
    signature: SignatureBytes,
) -> Result<bool> {
    let pk: PublicKey = pubkey.try_into()?;
    let sg: Signature = signature.try_into()?;

    Ok(sg.verify(pk, message))
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom as _;

    use bls::{SecretKey, SecretKeyBytes};

    use super::*;

    #[test]
    fn test_bls_verify_simple() {
        // Load some keys from a serialized secret key.
        let sk = SecretKey::try_from(SecretKeyBytes::from(*b"????????????????????????????????"))
            .expect("bytes represent a valid secret key");
        let pk = sk.to_public_key();

        // Sign a message
        let message = b"cats";
        let signature = sk.sign(message);
        assert!(signature.verify(pk, message));

        let pk_bytes = pk.into();
        let sg_bytes = signature.into();

        assert!(matches!(bls_verify(pk_bytes, message, sg_bytes), Ok(true)));
    }

    #[test]
    fn test_bls_verify_invalid_pubkey() {
        // Load some keys from a serialized secret key.
        let sk = SecretKey::try_from(SecretKeyBytes::from(*b"????????????????????????????????"))
            .expect("bytes represent a valid secret key");
        // Sign a message
        let message = b"cats";
        let signature = sk.sign(message);

        let pk_bytes = PublicKeyBytes::zero();
        let sg_bytes = signature.into();

        assert!(bls_verify(pk_bytes, message, sg_bytes).is_err());
    }

    #[test]
    fn test_bls_verify_invalid_sig() {
        // Load some keys from a serialized secret key.
        let sk = SecretKey::try_from(SecretKeyBytes::from(*b"????????????????????????????????"))
            .expect("bytes represent a valid secret key");
        let pk = sk.to_public_key();

        let pk_bytes = pk.into();
        let sg_bytes = SignatureBytes([1; 96]);

        assert!(bls_verify(pk_bytes, b"aaabbb", sg_bytes).is_err());
    }
}
