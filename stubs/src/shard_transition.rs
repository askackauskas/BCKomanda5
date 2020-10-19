use types::{config::Config, containers::{ShardBlock, ShardState}};


pub fn process_shard_block<C: Config>(shard_state: &mut ShardState, block: &ShardBlock<C>) {}