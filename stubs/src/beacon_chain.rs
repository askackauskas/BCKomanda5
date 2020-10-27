use anyhow::Result;
use ethereum_types::H256;
use std::collections::HashSet;
use std::ptr::eq;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::AttestationData,
    primitives::{CommitteeIndex, Domain, DomainType, Epoch, Gwei, Shard, Slot, ValidatorIndex},
};

const SHARD_BLOCK_OFFSETS: [i32; 12] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233];

pub fn get_online_validator_indices<C: Config>(_state: &BeaconState<C>) -> HashSet<ValidatorIndex> {
    let mut set_of_validator_indexes = HashSet::<ValidatorIndex>::new();

    // push anything depending on your needs while implementing
    set_of_validator_indexes.insert(1 as u64);
    set_of_validator_indexes.insert(2 as u64);

    set_of_validator_indexes
}

pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        return slot - 1;
    } else {
        return slot;
    }
}

pub fn optional_aggregate_verify(
    pubkeys: Vec<bls::PublicKeyBytes>,
    messages: Vec<H256>,
    signature: bls::SignatureBytes,
) -> bool {
    true
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
pub fn compute_updated_gasprice(prev_gasprice: Gwei, shard_block_length: u64) -> Gwei {
    0
}

pub fn get_offset_slots<C: Config>(state: &BeaconState<C>, shard: Shard) -> Vec<Slot> {
    compute_offset_slots(get_latest_slot_for_shard(&state, shard), state.slot)
}

pub fn get_domain<C: Config>(
    state: &BeaconState<C>,
    domain_type: DomainType,
    epoch: Epoch,
) -> Domain {
    H256::default()
}

pub fn get_shard_proposer_index<C: Config>(
    beacon_state: &BeaconState<C>,
    slot: Slot,
    shard: Shard,
) -> ValidatorIndex {
    0
}

pub fn get_latest_slot_for_shard<C: Config>(state: &BeaconState<C>, shard: Shard) -> Slot {
    state.shard_states[shard as usize].slot
}
pub fn get_active_shard_count<C: Config>(_state: &BeaconState<C>) -> u64 {
    return 64;
}

pub fn compute_committee_source_epoch<C: Config>(_epoch: Epoch, _period: u64) -> Epoch {
    return 5 as Epoch;
}

pub fn compute_committee<C: Config>(
    _indices: Vec<ValidatorIndex>,
    _seed: &H256,
    _index: u64,
    _count: u64,
) -> Result<Vec<ValidatorIndex>> {
    let mut new_vec: Vec<ValidatorIndex> = Vec::new();
    new_vec.push(1 as ValidatorIndex);
    new_vec.push(2 as ValidatorIndex);
    new_vec.push(3 as ValidatorIndex);
    Ok(new_vec)
}

#[must_use]
pub fn get_base_reward<C: Config>(state: &BeaconState<C>, index: ValidatorIndex) -> Gwei {
    /*let total_balance = get_total_active_balance(&state);
    let effective_balance = state.validators[index].effective_balance;

    let base_reward = (effective_balance * BASE_REWARD_FACTOR / integer_squareroot(total_balance) / BASE_REWARDS_PER_EPOCH) as Gweil
    base_reward*/
    0 as Gwei
}

#[must_use]
pub fn get_total_active_balance<C: Config>(state: &BeaconState<C>) -> Gwei {
    /*
    Return the combined effective balance of the active validators.
    Note: ``get_total_balance`` returns ``EFFECTIVE_BALANCE_INCREMENT`` Gwei minimum to avoid divisions by zero.
    */
    //get_total_balance::<C>(state, set(get_active_validator_indices(state, get_current_epoch(state)))?)
    0 as Gwei
}
