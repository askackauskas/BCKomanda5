
pub const INITIAL_ACTIVE_SHARDS :u64 = 64;
 pub const SHARD_BLOCK_OFFSETS :Vec<u64> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];// idk ar cia gerai
 pub const MAX_SHARD_BLOCKS_PER_ATTESTATION :usize = SHARD_BLOCK_OFFSETS.len();
 pub const MAX_SHARDS: u64 = 1024;
 pub const MAX_VALIDATORS_PER_COMMITTEE: u64 = 2048;