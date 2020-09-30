pub mod forkchoice {
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
    }

    impl PartialEq for SignedShardBlock {
        fn eq(&self, other: &Self) -> bool {
            self.message == other.message
        }
    }
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
}

fn main() {
    println!("Hello, world!");
}

pub mod tests {

    use crate::forkchoice::*;
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
            signed_blocks: signed_blocks,
            block_states: block_states,
        };

        assert_eq!(
            get_forkchoice_shard_store(beacon_state, test_shard),
            shard_store
        );
    }
}
