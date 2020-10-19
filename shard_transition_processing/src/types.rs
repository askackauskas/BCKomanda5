// TO DO: put types in a separate crate

pub type Shard = u64;
pub type Slot = u64;
pub type ComiteeIndex = u64;
pub type Root =  Bytes32;
pub type Epoch = u64;
use ethereum_types::H256;
use std::collections::HashMap;
use crate::configs::MAX_SHARDS;


// Signature
#[derive(Debug)]
pub enum SignatureBytes{
    Placeholder,
}

// Beacon State + Shard State types

pub struct BeaconState {
    pub slot: Slot,
    pub shard_states: [ShardState; MAX_SHARDS as usize],
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
    pub signature: SignatureBytes,
}

impl PartialEq for SignedShardBlock {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
}
pub struct ShardTransition{
    start_slot: Slot,
    shard_block_lenghts: Vec<u64>,
    shard_data_roots: Bytes32,
    shard_states: Vec<ShardState>,
    proposer_signature_aggregate: u64// BLSSignature turetu but
}
pub struct Attestation{
    aggregation_bits: Vec<u64>,//cia bitlist turetu but
    data: AttestationData,
    signature:u64// BLSSignature turetu but
}
pub struct AttestationData{
    slot:Slot,
    index:ComiteeIndex,
    beacon_block_root:Root,
    source: Checkpoint,
    target: Checkpoint,
    shard: Shard,
    shard_head_root: Root,
    shard_transition_root: Root
}
struct Checkpoint{
    epoch: Epoch,
    root: Root
}



pub struct Bytes32 {// daug kur naudoja Bytes32 tipa, tai toki radau nete ruste apsirasyt, bet ten lifetime buvo priskirtas, istryniau ji, nes nelabai dar suprantu kaip tai veikia
    pub store: Vec<[ u8; 4]>,
}

