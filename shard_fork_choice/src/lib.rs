mod types;
mod config;

pub use types::*;
pub use config::*;
use std::collections::HashMap;

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        slot - 1
    } else {
        slot
    }
}

/* def get_forkchoice_shard_store(anchor_state: BeaconState, shard: Shard) -> ShardStore:
    return ShardStore(
        shard=shard,
        signed_blocks={
            anchor_state.shard_states[shard].latest_block_root: SignedShardBlock(
                message=ShardBlock(slot=compute_previous_slot(anchor_state.slot), shard=shard)
            )
        },
        block_states={anchor_state.shard_states[shard].latest_block_root: anchor_state.copy().shard_states[shard]},
    )*/

pub fn get_forkchoice_shard_store(anchor_state: &BeaconState, shard: Shard) -> ShardStore {
    let shard_block = ShardBlock {
        slot: compute_previous_slot(anchor_state.slot),
        shard,
    };
    let signed_shard_block = SignedShardBlock { message: shard_block, signature: SignatureBytes::Placeholder };
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

// This is for small unit tests, for large integration testing go to tests/ folder
#[cfg(test)]
mod tests {
    /*
    #[test]
    fn new_test() {
        assert_eq!(2 + 2, 4);
    }
    */
}