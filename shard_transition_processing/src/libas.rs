mod types;
pub use crate::types::*;
mod configs;
pub use crate::configs::*;
mod constants;
pub use crate::constants::*;


pub fn proccess_shard_transitions(state:BeaconState, shard_transitions:ShardTransition,attestations:Attestation){// cia kaip sequence reiktu paduot
if compute_previous_slot(state.slot)>GENESIS_SLOT{
    proccess_crosslinks(state,shard_transitions,attestations)
}
assert!(verify_empty_shard_transitions(state,shard_transitions));

}
pub fn verify_empty_shard_transitions(state:BeaconState, shard_transitions:ShardTransition)->bool{
    
    for shard  in 1..get_active_shard_count(state)
    {

    if state.shard_state.slot != compute_previous_slot(state.slot){// nera shard_state lauko BeaconState
        
        if shard_transitions != ShardTransitions(){ // neradau ShardTransitions funckijos
            return false
        }

      }
        
    }
    true
}

fn compute_previous_slot(slot:Slot) -> Slot{
    if slot > 0{
        return slot-1
    }
    else{
        return slot
    }

}
fn get_active_shard_count(state:BeaconState) -> u64{
    return configs::INITIAL_ACTIVE_SHARDS
}
fn proccess_crosslinks(state:BeaconState, shard_transitions:ShardTransition,attestations:Attestation){

}


pub const INITIAL_ACTIVE_SHARDS :u64 = 64;
 pub const SHARD_BLOCK_OFFSETS :Vec<u64> = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];// idk ar cia gerai
 pub const MAX_SHARD_BLOCKS_PER_ATTESTATION :usize = SHARD_BLOCK_OFFSETS.len();
 pub const MAX_SHARDS: u64 = 1024;
 pub const MAX_VALIDATORS_PER_COMMITTEE: u64 = 2048;

 pub use crate::types::*;
pub const GENESIS_SLOT: crate::types::Slot = 1;// cia turetu but priskirta pirmam Slotui

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

