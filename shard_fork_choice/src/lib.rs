use std::collections::HashMap;

pub type Shard = u64;
pub type Slot = u64;
pub use ethereum_types::H256;

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

#[derive(Debug, Default)]
pub struct ShardStore {
    pub shard: Shard,
    pub signed_blocks: HashMap<H256, SignedShardBlock>,
    pub block_states: HashMap<H256, ShardState>,
    pub latest_messages: i32,
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

impl ShardBlock {
    pub fn new(slot: Slot, shard: Shard) -> ShardBlock {
        ShardBlock {
            slot: slot,
            shard: shard,
        }
    }
}

#[derive(Debug)]
pub struct SignedShardBlock {
    pub message: ShardBlock,
}

impl PartialEq for SignedShardBlock {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}

impl SignedShardBlock {
    pub fn new(message: ShardBlock) -> SignedShardBlock {
        SignedShardBlock { message: message }
        //SignedShardBlock{message: message, signature: *YOUR SIGNATURE FUNCTION HERE*}
    }
}

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        slot - 1
    } else {
        slot
    }
}

pub fn get_forkchoice_shard_store(anchor_state: &BeaconState, shard: Shard) -> ShardStore {
    let shard_block = ShardBlock::new(compute_previous_slot(anchor_state.slot), shard);
    let signed_shard_block = SignedShardBlock::new(shard_block);
    let mut signed_blocks = HashMap::new();
    signed_blocks.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        signed_shard_block,
    );
    let mut block_states: HashMap<ethereum_types::H256, ShardState> = HashMap::new();
    block_states.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        anchor_state.clone().shard_states[shard as usize],
    );
    ShardStore {
        shard,
        signed_blocks: { signed_blocks },
        block_states: { block_states },
        ..Default::default()
    }
}

// This is for small unit tests
#[cfg(test)]
mod tests {
    /*
    #[test]
    fn new_test() {
        assert_eq!(2 + 2, 4);
    }
    */
}
