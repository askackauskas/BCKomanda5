// This is for large integration testing

use shard_fork_choice::*;
use std::collections::HashMap;

pub type Shard = u64;
pub type Slot = u64;
pub use ethereum_types::H256;

#[test]
fn get_forkchoice_shard_store_test() {
    let test_shard = 0;
    let shard_state = ShardState {
        shard: test_shard,
        latest_block_root: H256([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1,
        ]),
    };
    let beacon_state = BeaconState {
        slot: 2,
        shard_states: vec![shard_state],
    };
    let shard_block = ShardBlock {
        slot: 1,
        shard: test_shard,
    };
    let signed_shard_block = SignedShardBlock {
        message: shard_block,
    };
    let mut signed_blocks = HashMap::new();
    signed_blocks.insert(
        beacon_state.shard_states[test_shard as usize].latest_block_root,
        signed_shard_block,
    );
    let mut block_states: HashMap<ethereum_types::H256, ShardState> = HashMap::new();
    block_states.insert(
        beacon_state.shard_states[test_shard as usize].latest_block_root,
        beacon_state.clone().shard_states[test_shard as usize],
    );
    let shard_store = ShardStore {
        shard: test_shard,
        signed_blocks,
        block_states,
    };

    assert_eq!(
        get_forkchoice_shard_store(beacon_state, test_shard),
        shard_store
    );
}

/*
#[test]
fn new_test() {
    let ss = ShardState{shard: 2, latest_block_root: H256(1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1)};
    let bs = BeaconState{slot: 2, shard_states[]};
}
*/