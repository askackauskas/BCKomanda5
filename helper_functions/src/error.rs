use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("attestation signature is invalid")]
    AttestationSignatureInvalid,
    #[error("no attesting indices")]
    AttestingIndicesEmpty,
    #[error("attesting indices are not sorted and unique")]
    AttestingIndicesNotSortedAndUnique,
    #[error("index is out of bounds")]
    IndexOutOfBounds,
    #[error("slot is out of range")]
    SlotOutOfRange,
    #[error("no validator indices")]
    ValidatorIndicesEmpty,
}
