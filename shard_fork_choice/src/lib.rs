pub use std::collections::HashMap;
pub use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ShardBlock, SignedShardBlock, ShardState},
    primitives::{Shard, Slot, H256, Root},
};

#[derive(Debug, Default)]
pub struct ShardStore<C: Config> {
    pub shard: Shard,
    pub signed_blocks: HashMap<Root, SignedShardBlock<C>>,
    pub block_states: HashMap<Root, ShardState>,
    pub latest_messages: i32,
}

impl<C: Config> PartialEq for ShardStore<C> {
    fn eq(&self, other: &Self) -> bool {
        self.shard == other.shard
            && self.signed_blocks == other.signed_blocks
            && self.block_states == other.block_states
    }
}

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        slot - 1
    } else {
        slot
    }
}

/*
def get_forkchoice_shard_store(anchor_state: BeaconState, shard: Shard) -> ShardStore:
    return ShardStore(
        shard=shard,
        signed_blocks={
            anchor_state.shard_states[shard].latest_block_root: SignedShardBlock(
                message=ShardBlock(slot=compute_previous_slot(anchor_state.slot), shard=shard)
            )
        },
        block_states={anchor_state.shard_states[shard].latest_block_root: anchor_state.copy().shard_states[shard]},
    )
*/
pub fn get_forkchoice_shard_store<C: Config>(anchor_state: &BeaconState<C>, shard: Shard) -> ShardStore<C> {
    let shard_block = ShardBlock{
        beacon_parent_root: /* TODO */,
        body: /* TODO */,
        proposer_index: /* TODO */,
        shard_parent_root: /* TODO */,
        slot: compute_previous_slot(anchor_state.slot),
        shard};
    let signed_shard_block = SignedShardBlock{
        message: shard_block,
        signature: /* TODO */,
    };
    let signed_blocks = HashMap::new();
    signed_blocks.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        signed_shard_block,
    );

    let block_states: HashMap<H256, ShardState> = HashMap::new();
    block_states.insert(
        anchor_state.shard_states[shard as usize].latest_block_root,
        anchor_state.shard_states[shard as usize],
    );

    ShardStore {
        shard,
        signed_blocks,
        block_states,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::compute_previous_slot;
    use crate::Slot;
    #[test]
    fn test_compute_previous_slot() {
        let five: Slot = 5;
        let six: Slot = 6;
        let zero: Slot = 0;
        assert_eq!(compute_previous_slot(six), five);
        assert_eq!(compute_previous_slot(zero), zero);
    }
}
