use std::vec::Vec;
use anyhow::{ensure,Result};
use crate::process_crosslink_for_shard::process_crosslink_for_shard;
use super::error::Error;
use types::{
    beacon_state::BeaconState,
    consts::GENESIS_SLOT,
    containers::Attestation,
    config::Config,
    containers::ShardTransition,
    primitives::{Root}
};
use stubs::beacon_chain::{
    compute_previous_slot,
    get_active_shard_count,
    compute_shard_from_committee_index,
    get_committee_count_per_slot,
    is_on_time_attestation,
};
use helper_functions::{
    misc::{
        compute_epoch_at_slot
        },
        predicates::is_winning_attestation
};



pub fn proccess_shard_transitions<C: Config>(
    state:&mut BeaconState<C>,
    shard_transitions:Vec<ShardTransition<C>>,
    attestations: Vec<Attestation<C>>
) {
    if compute_previous_slot(state.slot)>GENESIS_SLOT{
        proccess_crosslinks(state,shard_transitions,attestations);
    }
    ensure!(verify_empty_shard_transitions(state, shard_transitions),
        Error::NotEmptyShardTransition{});
}

pub fn verify_empty_shard_transitions<C: Config>(state:&mut BeaconState<C>,  shard_transitions:Vec<ShardTransition<C>>)->bool{
    for shard  in 1..get_active_shard_count(state) {
        if (state.shard_states[shard as usize].slot != compute_previous_slot(state.slot) &&  shard_transitions[shard as usize] != ShardTransition{..Default::default()}) {
            return false
        }
    }
    true
}

fn proccess_crosslinks<C: Config>(
    state:&mut BeaconState<C>,
    shard_transitions:Vec<ShardTransition<C>>,
    attestations: Vec<Attestation<C>>
)-> Result<()> {
    let on_time_attestation_slot=compute_previous_slot(state.slot);
    let committee_count = get_committee_count_per_slot(state,compute_epoch_at_slot::<C>(on_time_attestation_slot));
    for committee_index in 0..committee_count { // nezinau ar cia gerai, ten mapina, o cia tiesiog praiteratinu, no idea...
        let shard = compute_shard_from_committee_index(state, committee_index, on_time_attestation_slot);
    
        let shard_attestations = Vec::new();
        for attestation in attestations{
            if is_on_time_attestation(state, &attestation.data) && attestation.data.index == committee_index{// cia gal kitaip attestation data reikia paduot, gal pati argumenta su referencu perduot
                shard_attestations.push(attestation)
            }
        }

        let winning_root = process_crosslink_for_shard(
            state, committee_index, shard_transitions[shard as usize], shard_attestations
        )?;

        if winning_root != Root::default(){
            // # Mark relevant pending attestations as creating a successful crosslink
            for pending_attestation in state.current_epoch_attestations.into_iter(){
                if is_winning_attestation(state, *pending_attestation, committee_index, winning_root) {
                    pending_attestation.crosslink_success = true;
                }
            }
        }    
    }
    Ok(())
}
