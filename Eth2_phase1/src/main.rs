use std::collections::HashMap;

pub type Shard = u64;
pub type Slot = u64;
pub use ethereum_types::H256;

#[derive(Clone)]
pub struct BeaconState {
    pub slot: Slot,
    pub shard_states: Vec<ShardState>, //should be VariableList??
}

impl Clone for BeaconState {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct ShardState {
    pub shard: Shard,
    pub latest_block_root: H256,
}

impl Clone for ShardState {
    fn clone(&self) -> Self {
        *self
    }
}

pub struct ShardStore {
    pub shard: Shard,
    pub signed_blocks: HashMap<H256, SignedShardBlock>,
    pub block_states: HashMap<H256, ShardState>,
}

pub struct ShardBlock {
    slot: Slot,
    shard: Shard,
}

pub struct SignedShardBlock {
    message: ShardBlock,
    signature: u64, //FAKE-BLS
}
pub fn compute_previous_slot(slot: Slot) -> Slot {
    slot
}
pub fn get_forkchoice_shard_store(anchor_state: BeaconState, shard: Shard) -> ShardStore {
    let sb = ShardBlock {
        slot: compute_previous_slot(anchor_state.slot),
        shard: shard,
    };
    let ssb = SignedShardBlock {
        message: sb,
        signature: 1,
    };
    let mut map = HashMap::new();
    map.insert(
        anchor_state
            .shard_states
            .iter()
            .find(|&&x| shard == shard)
            .unwrap()
            .latest_block_root,
        ssb,
    );
    let mut map2: HashMap<ethereum_types::H256, ShardState> = HashMap::new();
    map2.insert(
        anchor_state
            .shard_states
            .iter()
            .find(|&&x| shard == shard)
            .unwrap()
            .latest_block_root,
        anchor_state
            .clone()
            .shard_states()
            .iter()
            .find(|&&x| shard == shard)
            .unwrap(),
    );
    ShardStore {
        shard: shard,
        signed_blocks: { map },
        block_states: { map2 },
    }
}

fn main() {
    println!("Hello, world!");
}

//    ShardStore {
//        shard: shard,
//        signed_blocks: vec![ssb],
//        block_states: vec![vec![]],
//    }
