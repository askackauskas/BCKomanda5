mod error;

use anyhow::{ensure, Result};
use error::Error;
use helper_functions::{
    accessors::{
        get_attesting_indices, get_beacon_committee, get_beacon_proposer_index,
        get_block_root_at_slot, get_total_balance,
    },
    misc::{compute_epoch_at_slot, compute_previous_slot, compute_signing_root},
    mutators::{decrease_balance, increase_balance},
    predicates::is_winning_attestation,
};
use itertools::izip;
use std::collections::{BTreeSet, HashSet};
use std::iter::FromIterator;
use std::vec::Vec;
use stubs::beacon_chain::{
    compute_shard_from_committee_index, compute_updated_gasprice, get_active_shard_count,
    get_base_reward, get_committee_count_per_slot, get_domain, get_offset_slots,
    get_online_validator_indices, get_shard_proposer_index, is_on_time_attestation,
    optional_aggregate_verify,
};
use tree_hash::TreeHash;
use types::{
    beacon_state::BeaconState,
    config::Config,
    consts::GENESIS_SLOT,
    containers::{Attestation, ShardBlockHeader, ShardTransition},
    primitives::{CommitteeIndex, Gwei, Root, Shard, ValidatorIndex, H256},
};

// TODO Should be implemented as a method (impl) for BeaconState?
pub fn apply_shard_transition<C: Config>(
    state: &mut BeaconState<C>,
    shard: Shard,
    transition: &ShardTransition<C>,
) -> Result<()> {
    ensure!(
        state.slot <= GENESIS_SLOT,
        Error::InvalidSlot {
            slot: state.slot,
            genesis_slot: GENESIS_SLOT,
        }
    );
    // Correct data root count
    let offset_slots = get_offset_slots(state, shard);
    ensure!(
        // TODO Find a shorter way to write this?
        offset_slots.len() == transition.shard_data_roots.len()
            && offset_slots.len() == transition.shard_states.len()
            && offset_slots.len() == transition.shard_block_lengths.len(),
        Error::IncorrectDataRootCount {
            offset_slots: offset_slots.len(),
            shard_data_roots: transition.shard_data_roots.len(),
            shard_states: transition.shard_states.len(),
            shard_block_lengths: transition.shard_block_lengths.len(),
        }
    );
    ensure!(
        transition.start_slot == offset_slots[0],
        Error::IncorrectSlot {
            slot: transition.start_slot,
            expected_slot: offset_slots[0],
        }
    );

    let headers = Vec::new();
    let proposers = Vec::new();
    let mut prev_gasprice = state.shard_states[shard as usize].gasprice;
    let mut shard_parent_root = state.shard_states[shard as usize].latest_block_root;
    for (i, offset_slot) in offset_slots.iter().enumerate() {
        let shard_block_length = transition.shard_block_lengths[i];
        let shard_state = transition.shard_states[i];

        // Verify correct calculation of gas prices and slots
        ensure!(
            shard_state.gasprice == compute_updated_gasprice(prev_gasprice, shard_block_length),
            Error::IncorrectGasprice {
                gasprice: shard_state.gasprice,
                expected_gasprice: compute_updated_gasprice(prev_gasprice, shard_block_length),
            }
        );
        ensure!(
            shard_state.slot == *offset_slot,
            Error::IncorrectSlot {
                slot: shard_state.slot,
                expected_slot: *offset_slot,
            }
        );

        // Collect the non-empty proposals result
        match shard_block_length {
            0 => {
                // Must have a stub for `shard_data_root` if empty slot
                ensure!(
                    transition.shard_data_roots[i] == H256::default(),
                    Error::NonEmptyRoot {
                        root: transition.shard_data_roots[i]
                    }
                );
            }
            _ => {
                let proposer_index = get_shard_proposer_index(&*state, *offset_slot, shard);
                // Reconstruct shard headers
                let header = ShardBlockHeader {
                    shard_parent_root,
                    beacon_parent_root: get_block_root_at_slot(state, *offset_slot)?,
                    slot: *offset_slot,
                    shard,
                    proposer_index,
                    body_root: transition.shard_data_roots[i],
                };
                shard_parent_root = header.tree_hash_root();
                headers.push(header);
                proposers.push(proposer_index);
            }
        }
        prev_gasprice = shard_state.gasprice;
    }

    let pubkeys = Vec::new();
    let signing_roots = Vec::new();
    for proposer in proposers {
        pubkeys.push(state.validators[proposer as usize].pubkey);
    }
    for header in headers {
        /* TODO: Create new const DOMAIN_SHARD_PROPOSAL in Config */
        signing_roots.push(compute_signing_root(
            &header,
            get_domain(
                state,
                C::DOMAIN_SHARD_PROPOSAL,
                compute_epoch_at_slot::<C>(header.slot),
            ),
        ));
    }

    // Verify combined proposer signature
    ensure!(
        optional_aggregate_verify(
            pubkeys,
            signing_roots,
            transition.proposer_signature_aggregate
        ),
        Error::UnverifiedAggregateSignature {}
    );

    // Copy and save updated shard state
    let shard_state = transition.shard_states[transition.shard_states.len() - 1].clone();
    shard_state.slot = compute_previous_slot(state.slot);
    state.shard_states[shard as usize] = shard_state;

    Ok(())
}

pub fn process_crosslink_for_shard<C: Config>(
    state: &mut BeaconState<C>,
    committee_index: CommitteeIndex,
    shard_transition: ShardTransition<C>,
    attestations: Vec<Attestation<C>>,
) -> Result<Root> {
    let on_time_attestation_slot = compute_previous_slot(state.slot);
    let committee = HashSet::from_iter(get_beacon_committee(
        state,
        on_time_attestation_slot,
        committee_index,
    )?);
    let online_indices = get_online_validator_indices(state);
    let shard =
        compute_shard_from_committee_index(state, committee_index, on_time_attestation_slot);

    // Loop over all shard transition roots
    let shard_transition_roots = BTreeSet::new();
    for a in attestations {
        shard_transition_roots.insert(a.data.shard_transition_root);
    }
    for shard_transition_root in shard_transition_roots {
        let transition_attestations = Vec::new();
        for a in attestations {
            if a.data.shard_transition_root == shard_transition_root {
                transition_attestations.push(a);
            }
        }
        let mut transition_participants = HashSet::<ValidatorIndex>::new();
        for attestation in transition_attestations {
            let participants: Vec<ValidatorIndex> =
                get_attesting_indices(state, &attestation.data, &attestation.aggregation_bits)?
                    .collect();
            transition_participants.extend(&participants);
        }

        let stake = get_total_balance(state, online_indices.intersection(&transition_participants))?
            as u64
            * 3;
        let min_stake =
            get_total_balance(state, online_indices.intersection(&committee).collect())? as u64 * 2;
        // If not enough stake, try next transition root
        if stake < min_stake {
            continue;
        }

        // Attestation <-> shard transition consistency
        ensure!(
            shard_transition_root == shard_transition.tree_hash_root(),
            Error::IncorrectRoot {
                root: shard_transition_root,
                expected_root: shard_transition.tree_hash_root()
            }
        );

        // Check `shard_head_root` of the winning root
        let last_offset_index = shard_transition.shard_states.len() - 1;
        let shard_head_root = shard_transition.shard_states[last_offset_index].latest_block_root;
        for attestation in transition_attestations {
            ensure!(
                attestation.data.shard_head_root == shard_head_root,
                Error::IncorrectRoot {
                    root: attestation.data.shard_head_root,
                    expected_root: shard_head_root
                }
            );
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
    ensure!(
        shard_transition
            == ShardTransition {
                ..Default::default()
            },
        Error::MissingTransitionRoot {}
    );
    Ok(Root {
        ..Default::default()
    })
}

pub fn proccess_shard_transitions<C: Config>(
    state: &mut BeaconState<C>,
    shard_transitions: Vec<ShardTransition<C>>,
    attestations: Vec<Attestation<C>>,
) {
    if compute_previous_slot(state.slot) > GENESIS_SLOT {
        proccess_crosslinks(state, shard_transitions, attestations);
    }
    ensure!(
        verify_empty_shard_transitions(state, shard_transitions),
        Error::NotEmptyShardTransition {}
    );
}

pub fn verify_empty_shard_transitions<C: Config>(
    state: &mut BeaconState<C>,
    shard_transitions: Vec<ShardTransition<C>>,
) -> bool {
    for shard in 1..get_active_shard_count(state) {
        if (state.shard_states[shard as usize].slot != compute_previous_slot(state.slot)
            && shard_transitions[shard as usize]
                != ShardTransition {
                    ..Default::default()
                })
        {
            return false;
        }
    }
    true
}

fn proccess_crosslinks<C: Config>(
    state: &mut BeaconState<C>,
    shard_transitions: Vec<ShardTransition<C>>,
    attestations: Vec<Attestation<C>>,
) -> Result<()> {
    let on_time_attestation_slot = compute_previous_slot(state.slot);
    let committee_count =
        get_committee_count_per_slot(state, compute_epoch_at_slot::<C>(on_time_attestation_slot));
    for committee_index in 0..committee_count {
        // nezinau ar cia gerai, ten mapina, o cia tiesiog praiteratinu, no idea...
        let shard =
            compute_shard_from_committee_index(state, committee_index, on_time_attestation_slot);

        let shard_attestations = Vec::new();
        for attestation in attestations {
            if is_on_time_attestation(state, &attestation.data)
                && attestation.data.index == committee_index
            {
                // cia gal kitaip attestation data reikia paduot, gal pati argumenta su referencu perduot
                shard_attestations.push(attestation)
            }
        }

        let winning_root = process_crosslink_for_shard(
            state,
            committee_index,
            shard_transitions[shard as usize],
            shard_attestations,
        )?;

        if winning_root != Root::default() {
            // # Mark relevant pending attestations as creating a successful crosslink
            for pending_attestation in state.current_epoch_attestations.into_iter() {
                if is_winning_attestation(
                    state,
                    *pending_attestation,
                    committee_index,
                    winning_root,
                ) {
                    pending_attestation.crosslink_success = true;
                }
            }
        }
    }
    Ok(())
}
