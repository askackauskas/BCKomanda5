use types::primitives::{Slot, Gwei, H256};
use thiserror::Error;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
pub enum Error/*<C: Config>*/ {
    #[error("Invalid slot ({slot} <= {genesis_slot})")]
    InvalidSlot {
        slot: Slot,
        genesis_slot: Slot },
    #[error("Invalid # of transition data roots ({offset_slots}, {shard_data_roots}, {shard_states}, {shard_block_lengths})")]
    IncorrectDataRootCount{
        offset_slots : usize,
        shard_data_roots: usize,
        shard_states: usize,
        shard_block_lengths: usize },
    #[error("Incorrect slot ({slot} != {expected_slot})")]
    IncorrectSlot {
        slot: Slot,
        expected_slot: Slot },
    #[error("Incorrect gasprice ({gasprice} != {expected_gasprice})")]
    IncorrectGasprice{
        gasprice: Gwei,
        expected_gasprice: Gwei },
    #[error("Expected empty root ({root})")]
    NonEmptyRoot{ root: H256 },
    #[error("Aggregate signature verification for shard transition has failed!")]
    UnverifiedAggregateSignature{},
}