mod types;
use types::*;
use std::collections::HashMap;

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        slot - 1
    } else {
        slot
    }
}

pub fn get_forkchoice_shard_store(anchor_state: BeaconState, shard: Shard) -> ShardStore {
    let sb = ShardBlock {
        slot: compute_previous_slot(anchor_state.slot),
        shard,
    };
    let ssb = SignedShardBlock { message: sb };
    let mut map = HashMap::new();
    map.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        ssb,
    );
    let mut map2: HashMap<ethereum_types::H256, ShardState> = HashMap::new();
    map2.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        anchor_state.clone().shard_states[shard as usize],
    );
    ShardStore {
        shard,
        signed_blocks: { map },
        block_states: { map2 },
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