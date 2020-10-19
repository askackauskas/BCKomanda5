// `BlsError` does not implement `std::error::Error`.

use bls_eth_rust::BlsError;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("BLS deserialization failed: {0:?}")]
pub struct Error(pub(crate) BlsError);
