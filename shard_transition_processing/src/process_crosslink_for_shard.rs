use std::vec::Vec;
use std::collections::{BTreeSet,HashSet};
use std::iter::FromIterator;
use itertools::izip;
use anyhow::{ensure,Result};
use crate::error::Error;
use tree_hash::TreeHash;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ShardTransition, Attestation},
    primitives::{CommitteeIndex, Root, ValidatorIndex, Gwei},
};
use helper_functions::{
    misc::compute_previous_slot,
    accessors::{
        get_beacon_committee,
        get_attesting_indices,
        get_beacon_proposer_index,
        get_total_balance},
    mutators::{increase_balance,decrease_balance},
};
/* TODO add all these stubs to Stubs crate and create a commit/pull request */
use stubs::beacon_chain::{
    get_online_validator_indices,
    compute_shard_from_committee_index,
    get_base_reward,
    get_offset_slots,
    get_shard_proposer_index,
};
use crate::apply_shard_transition::apply_shard_transition;

pub fn process_crosslink_for_shard<C: Config>(state: &mut BeaconState<C>,
                                committee_index: CommitteeIndex,
                                shard_transition: ShardTransition<C>,
                                attestations: Vec<Attestation<C>>) -> Result<Root>
{
    let on_time_attestation_slot = compute_previous_slot(state.slot);
    let committee = HashSet::from_iter(get_beacon_committee(state, on_time_attestation_slot, committee_index)?);
    let online_indices = get_online_validator_indices(state);
    let shard = compute_shard_from_committee_index(state, committee_index, on_time_attestation_slot);

    // Loop over all shard transition roots
    let shard_transition_roots = BTreeSet::new();
    for a in attestations {
        shard_transition_roots.insert(a.data.shard_transition_root);
    }
    for shard_transition_root in shard_transition_roots {
        let transition_attestations = Vec::new();
        for a in attestations{
            if a.data.shard_transition_root == shard_transition_root {
                transition_attestations.push(a);
            }
        }
        let mut transition_participants = HashSet::<ValidatorIndex>::new();
        for attestation in transition_attestations {
            let participants: Vec::<ValidatorIndex> = get_attesting_indices(state, &attestation.data, &attestation.aggregation_bits)?.collect();
            transition_participants.extend(&participants);
        }

        let stake = get_total_balance(state, online_indices.intersection(&transition_participants))? as u64 * 3;
        let min_stake = get_total_balance(state, online_indices.intersection(&committee).collect())? as u64 * 2;
        // If not enough stake, try next transition root
        if stake < min_stake {
            continue;
        }

        // Attestation <-> shard transition consistency
        ensure!(
            shard_transition_root == shard_transition.tree_hash_root(),
            Error::IncorrectRoot{
                root: shard_transition_root,
                expected_root: shard_transition.tree_hash_root()
            });

        // Check `shard_head_root` of the winning root
        let last_offset_index = shard_transition.shard_states.len() - 1;
        let shard_head_root = shard_transition.shard_states[last_offset_index].latest_block_root;
        for attestation in transition_attestations {
            ensure!(attestation.data.shard_head_root == shard_head_root,
                Error::IncorrectRoot{
                    root: attestation.data.shard_head_root,
                    expected_root: shard_head_root});
        }

        // Apply transition
        apply_shard_transition(&mut state, shard, &shard_transition);
        // Apply proposer reward and cost
        let beacon_proposer_index = get_beacon_proposer_index(&state)?;
        let mut estimated_attester_reward = 0;
        for attester in transition_participants {
            estimated_attester_reward += get_base_reward(&state, attester);
        }
        let proposer_reward = (estimated_attester_reward / C::PROPOSER_REWARD_QUOTIENT) as Gwei;
        increase_balance(state, beacon_proposer_index, proposer_reward);
        let states_slots_lengths = izip!(
            shard_transition.shard_states.iter(),
            get_offset_slots(&state, shard),
            shard_transition.shard_block_lengths.iter()
        );
        for (shard_state, slot, length) in states_slots_lengths {
            let proposer_index = get_shard_proposer_index(state, slot, shard);
            decrease_balance(state, proposer_index, shard_state.gasprice * length);
        }

        // Return winning transition root
        return Ok(shard_transition_root);
    }

    // No winning transition root, ensure empty and return empty root
    ensure!(shard_transition == ShardTransition{..Default::default()},
        Error::MissingTransitionRoot{});
    Ok(Root{..Default::default()})
}