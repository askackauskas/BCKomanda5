// TODO: Re-write test


/*use std::collections::HashMap;
use types::{
    beacon_state::BeaconState,
    containers::{ShardState, ShardBlock, SignedShardBlock},
    primitives::{H256},
};

#[test]
fn get_forkchoice_shard_store_test() {
    let test_shard = 0;
    let shard_state = ShardState {
        slot: ,
        gasprice: ,
        latest_block_root: H256([
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1,
        ]),
    };
    let beacon_state = BeaconState {};
    let shard_block = ShardBlock::new(1, test_shard);
    let signed_shard_block = SignedShardBlock::new(shard_block);
    let mut signed_blocks = HashMap::new();
    signed_blocks.insert(
        beacon_state.shard_states[test_shard as usize].latest_block_root,
        signed_shard_block,
    );
    let mut block_states: HashMap<H256, ShardState> = HashMap::new();
    block_states.insert(
        beacon_state.shard_states[test_shard as usize].latest_block_root,
        beacon_state.shard_states[test_shard as usize],
    );
    let shard_store = ShardStore {
        shard: test_shard,
        signed_blocks,
        block_states,
        ..Default::default()
    };

    assert_eq!(
        get_forkchoice_shard_store<MainnetConfig>(&beacon_state, test_shard),
        shard_store
    );
}
*/