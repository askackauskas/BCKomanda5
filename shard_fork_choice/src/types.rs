// TO DO: put types in a separate crate

pub type Shard = u64;
pub type Slot = u64;
use ethereum_types::H256;
use std::collections::HashMap;
use crate::config::MAX_SHARDS;

// Signature
#[derive(Debug)]
pub enum SignatureBytes{
    Placeholder,
}

// Beacon State + Shard State types

pub struct BeaconState {
    pub slot: Slot,
    pub shard_states: [ShardState; MAX_SHARDS as usize],
}

#[derive(Clone, Copy, Debug)]
pub struct ShardState {
    pub shard: Shard,
    pub latest_block_root: H256,
}

impl PartialEq for ShardState {
    fn eq(&self, other: &Self) -> bool {
        self.shard == other.shard && self.latest_block_root == other.latest_block_root
    }
}

#[derive(Debug, Default)]
pub struct ShardStore {
    pub shard: Shard,
    pub signed_blocks: HashMap<H256, SignedShardBlock>,
    pub block_states: HashMap<H256, ShardState>,
    pub latest_messages: i32
}

impl PartialEq for ShardStore {
    fn eq(&self, other: &Self) -> bool {
        self.shard == other.shard
            && self.signed_blocks == other.signed_blocks
            && self.block_states == other.block_states
    }
}

#[derive(Debug)]
pub struct ShardBlock {
    pub slot: Slot,
    pub shard: Shard,
}

impl PartialEq for ShardBlock {
    fn eq(&self, other: &Self) -> bool {
        self.slot == other.slot && self.shard == other.shard
    }
}

#[derive(Debug)]
pub struct SignedShardBlock {
    pub message: ShardBlock,
    pub signature: SignatureBytes,
}

impl PartialEq for SignedShardBlock {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}
