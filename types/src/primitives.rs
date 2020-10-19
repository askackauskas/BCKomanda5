use bls::{PublicKey, Signature, SignatureBytes};
use derive_more::{DebugCustom, From, Into};
use ethereum_types::H32;
use serde::{Deserialize, Serialize};
use ssz_new::types::FixedVector;
use ssz_new_derive::{SszDecode, SszEncode};
use tree_hash_derive::TreeHash;
use typenum::{Sum, U1};

use crate::consts::DepositContractTreeDepth;

pub use ethereum_types::{H160, H256};

pub type AggregatePublicKey = PublicKey;
pub type AggregateSignature = Signature;
pub type AggregateSignatureBytes = SignatureBytes;
pub type CommitteeIndex = u64;
pub type DepositIndex = u64;
pub type DepositProof = FixedVector<H256, Sum<DepositContractTreeDepth, U1>>;
pub type Domain = H256;
pub type DomainType = u32;
pub type Epoch = u64;
pub type Eth1Address = H160;
pub type Eth1BlockHash = H256;
pub type Eth1BlockNumber = u64;
pub type Eth1TransactionHash = H256;
pub type ForkDigest = H32;
pub type Gwei = u64;
pub type Slot = u64;
pub type SubnetId = u64;
pub type UnixSeconds = u64;
pub type ValidatorIndex = u64;
pub type Shard = u64;
pub type OnlineEpochs = u8;
pub type Root = H256;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    From,
    Into,
    DebugCustom,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
#[debug(fmt = "{:?}", "H32(*bytes)")]
// Specification tests represent `Version` with strings of the form "0x…". `H32` has the
// `Deserialize` and `Serialize` impls needed, but SSZ traits are not implemented for it.
#[serde(from = "H32", into = "H32")]
pub struct Version {
    // SSZ derive macros only work on named fields.
    bytes: [u8; 4],
}

impl From<H32> for Version {
    fn from(hash: H32) -> Self {
        Self { bytes: hash.into() }
    }
}

impl From<Version> for H32 {
    fn from(version: Version) -> Self {
        version.bytes.into()
    }
}

impl Version {
    pub(crate) const fn new(bytes: [u8; 4]) -> Self {
        Self { bytes }
    }
}
