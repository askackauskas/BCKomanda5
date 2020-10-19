use std::ptr::eq;
use types::{
    beacon_state::BeaconState, config::Config, containers::AttestationData,
    primitives::CommitteeIndex, primitives::Epoch, primitives::Shard, primitives::Slot,
    primitives::ValidatorIndex,
};
use ethereum_types::H256;
use anyhow::Result;

const SHARD_BLOCK_OFFSETS: [i32; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

pub fn get_online_validator_indices<C: Config>(_state: &BeaconState<C>) -> Vec<ValidatorIndex> {
    let mut set_of_validator_indexes = Vec::<ValidatorIndex>::new();

    // push anything depending on your needs while implementing
    set_of_validator_indexes.push(1 as u64);
    set_of_validator_indexes.push(2 as u64);

    set_of_validator_indexes
}

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        return slot - 1;
    } else {
        return slot;
    }
}

pub fn get_committee_count_per_slot<C: Config>(_state: &BeaconState<C>, _epoch: Epoch) -> u64 {
    10 as u64
}

pub fn compute_shard_from_committee_index<C: Config>(
    _state: &BeaconState<C>,
    _index: CommitteeIndex,
    _slot: Slot,
) -> Shard {
    10 as Shard
}

pub fn is_on_time_attestation<C: Config>(
    state: &BeaconState<C>,
    attestation_data: &AttestationData,
) -> bool {
    return eq(&attestation_data.slot, &compute_previous_slot(state.slot));
}

pub fn compute_offset_slots(start_slot: Slot, end_slot: Slot) -> Vec<u64> {
    let mut v = Vec::<Slot>::new();
    for x in SHARD_BLOCK_OFFSETS.iter() {
        if start_slot + (*x as Slot) < end_slot {
            v.push(start_slot + (*x as Slot));
        }
    }
    v
}

pub fn get_latest_slot_for_shard<C: Config>(state: &BeaconState<C>, shard: Shard) -> Slot {
    state.shard_states[shard as usize].slot
}
pub fn get_active_shard_count<C: Config>(_state: &BeaconState<C>) -> u64 {
    return 64;
}

pub fn compute_committee_source_epoch<C: Config>(
    _epoch: Epoch,
    _period: u64,
) -> Epoch {
    return 5 as Epoch
}

pub fn compute_committee<C: Config>(
    _indices: Vec<ValidatorIndex>,
    _seed: &H256,
    _index: u64,
    _count: u64,
) -> Result<Vec<ValidatorIndex>> {
    let mut new_vec : Vec<ValidatorIndex> = Vec::new();
    new_vec.push(1 as ValidatorIndex);
    new_vec.push(2 as ValidatorIndex);
    new_vec.push(3 as ValidatorIndex);
    Ok(new_vec)
}