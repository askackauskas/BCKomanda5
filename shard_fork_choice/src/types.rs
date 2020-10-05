pub type Shard = u64;
pub type Slot = u64;
use ethereum_types::H256;
use std::collections::HashMap;

#[derive(Clone)]
pub struct BeaconState {
    pub slot: Slot,
    pub shard_states: Vec<ShardState>, //should be VariableList??
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

#[derive(Debug)]
pub struct ShardStore {
    pub shard: Shard,
    pub signed_blocks: HashMap<H256, SignedShardBlock>,
    pub block_states: HashMap<H256, ShardState>,
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
    //signature: BLSSignature,
}

impl PartialEq for SignedShardBlock {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}
