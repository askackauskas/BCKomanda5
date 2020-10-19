use std::vec::Vec;
use anyhow::{ensure,Result};
use thiserror::Error;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ShardTransition, Attestation},
    primitives::{CommitteeIndex, Root, Slot},
    //consts::,
};
use helper_functions::{
    
};
/* TODO add all these stubs to Stubs crate and create a commit/pull request */
use stubs::beacon_chain::{
    
};

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Error)]
enum Error/*<C: Config>*/ {
    #[error("Invalid slot ({slot} <= {genesis_slot})")]
    InvalidSlot {
        slot: Slot,
        genesis_slot: Slot },
}

pub fn process_crosslink_for_shard<C: Config>(state: &BeaconState<C>,
                                committee_index: CommitteeIndex,
                                shard_transition: ShardTransition<C>,
                                attestations: Vec<Attestation<C>>) -> Result<Root>
{
    Ok(Root::new())
}

/*
    on_time_attestation_slot = compute_previous_slot(state.slot)
    committee = get_beacon_committee(state, on_time_attestation_slot, committee_index)
    online_indices = get_online_validator_indices(state)
    shard = compute_shard_from_committee_index(state, committee_index, on_time_attestation_slot)

    # Loop over all shard transition roots
    shard_transition_roots = set([a.data.shard_transition_root for a in attestations])
    for shard_transition_root in sorted(shard_transition_roots):
        transition_attestations = [a for a in attestations if a.data.shard_transition_root == shard_transition_root]
        transition_participants: Set[ValidatorIndex] = set()
        for attestation in transition_attestations:
            participants = get_attesting_indices(state, attestation.data, attestation.aggregation_bits)
            transition_participants = transition_participants.union(participants)

        enough_online_stake = (
            get_total_balance(state, online_indices.intersection(transition_participants)) * 3 >=
            get_total_balance(state, online_indices.intersection(committee)) * 2
        )
        # If not enough stake, try next transition root
        if not enough_online_stake:
            continue

        # Attestation <-> shard transition consistency
        assert shard_transition_root == hash_tree_root(shard_transition)

        # Check `shard_head_root` of the winning root
        last_offset_index = len(shard_transition.shard_states) - 1
        shard_head_root = shard_transition.shard_states[last_offset_index].latest_block_root
        for attestation in transition_attestations:
            assert attestation.data.shard_head_root == shard_head_root

        # Apply transition
        apply_shard_transition(state, shard, shard_transition)
        # Apply proposer reward and cost
        beacon_proposer_index = get_beacon_proposer_index(state)
        estimated_attester_reward = sum([get_base_reward(state, attester) for attester in transition_participants])
        proposer_reward = Gwei(estimated_attester_reward // PROPOSER_REWARD_QUOTIENT)
        increase_balance(state, beacon_proposer_index, proposer_reward)
        states_slots_lengths = zip(
            shard_transition.shard_states,
            get_offset_slots(state, shard),
            shard_transition.shard_block_lengths
        )
        for shard_state, slot, length in states_slots_lengths:
            proposer_index = get_shard_proposer_index(state, slot, shard)
            decrease_balance(state, proposer_index, shard_state.gasprice * length)

        # Return winning transition root
        return shard_transition_root

    # No winning transition root, ensure empty and return empty root
    assert shard_transition == ShardTransition()
    return Root()
    */