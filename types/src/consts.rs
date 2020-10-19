use typenum::{U32, U4, U64};

use crate::primitives::{Epoch, Slot};

pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const FAR_FUTURE_EPOCH: u64 = u64::MAX;
pub const GENESIS_EPOCH: Epoch = 0;
pub const GENESIS_SLOT: Slot = 0;

pub type AttestationSubnetCount = U64;
pub type DepositContractTreeDepth = U32;
pub type JustificationBitsLength = U4;
