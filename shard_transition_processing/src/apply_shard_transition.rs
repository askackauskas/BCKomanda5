use std::vec::Vec;
use tree_hash::TreeHash;
use anyhow::{ensure,Result};
use crate::error::Error;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ShardTransition, ShardBlockHeader},
    primitives::{Shard,H256},
    consts::GENESIS_SLOT,
};
use helper_functions::{
    accessors::get_block_root_at_slot,
    misc::{
        compute_previous_slot,
        compute_signing_root,
        compute_epoch_at_slot},
};
/* TODO add all these stubs to Stubs crate and create a commit/pull request */
use stubs::beacon_chain::{
    get_offset_slots,
    get_shard_proposer_index,
    compute_updated_gasprice,
    get_domain,
    optional_aggregate_verify,
};

// TODO Should be implemented as a method (impl) for BeaconState?
pub fn apply_shard_transition<C: Config>(state: &mut BeaconState<C>, shard: Shard, transition: &ShardTransition<C>) -> Result<()> {
    ensure!(
        state.slot <= GENESIS_SLOT,
        Error::InvalidSlot{
            slot: state.slot,
            genesis_slot: GENESIS_SLOT,
        });
    // Correct data root count
    let offset_slots = get_offset_slots(state, shard);
    ensure!( // TODO - shorter way to write?
        offset_slots.len() == transition.shard_data_roots.len() &&
        offset_slots.len() == transition.shard_states.len() &&
        offset_slots.len() == transition.shard_block_lengths.len(), 
        Error::IncorrectDataRootCount{
            offset_slots: offset_slots.len(),
            shard_data_roots: transition.shard_data_roots.len(),
            shard_states: transition.shard_states.len(),
            shard_block_lengths: transition.shard_block_lengths.len(),
        });
    ensure!(
        transition.start_slot == offset_slots[0],
        Error::IncorrectSlot{
            slot: transition.start_slot,
            expected_slot: offset_slots[0],
        });

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
            Error::IncorrectGasprice{
                gasprice: shard_state.gasprice,
                expected_gasprice: compute_updated_gasprice(prev_gasprice, shard_block_length),
            }); 
        ensure!(
            shard_state.slot == *offset_slot,
            Error::IncorrectSlot{
                slot: shard_state.slot,
                expected_slot: *offset_slot,
            });

        // Collect the non-empty proposals result
        match shard_block_length {
            0 => {
                // Must have a stub for `shard_data_root` if empty slot
                ensure!(
                    transition.shard_data_roots[i] == H256::default(),
                    Error::NonEmptyRoot{ root: transition.shard_data_roots[i] });
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
    for proposer in proposers{
        pubkeys.push(state.validators[proposer as usize].pubkey);
    }
    for header in headers {
        /* TODO: Create new const DOMAIN_SHARD_PROPOSAL in Config */ 
        signing_roots.push(compute_signing_root(&header, get_domain(state, C::DOMAIN_SHARD_PROPOSAL, compute_epoch_at_slot::<C>(header.slot))));
    }

    // Verify combined proposer signature
    ensure!(
        optional_aggregate_verify(pubkeys, signing_roots, transition.proposer_signature_aggregate),
        Error::UnverifiedAggregateSignature{});

    // Copy and save updated shard state
    let shard_state = transition.shard_states[transition.shard_states.len() - 1].clone();
    shard_state.slot = compute_previous_slot(state.slot);
    state.shard_states[shard as usize] = shard_state;

    Ok(())
}