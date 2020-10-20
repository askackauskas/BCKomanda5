use std::vec::Vec;
use anyhow::{ensure,Result};
use super::errors::Error;
use types::{
    beacon_state::BeaconState,
    consts::GENESIS_SLOT,
    containers::Attestation,
    config::Config,
    containers::ShardTransition,
    containers::ShardState

};
use stubs::beacon_chain::{
    compute_previous_slot,
    get_active_shard_count,
    get_committee_count_per_slot
};



pub fn proccess_shard_transitions<C: Config>(state: &BeaconState<C>, shard_transitions:Vec<ShardTransition<C>>,attestations: Vec<Attestation<C>>){
if compute_previous_slot(state.slot)>GENESIS_SLOT{
    proccess_crosslinks(state,shard_transitions,attestations)
}
ensure!(verify_empty_shard_transitions(state, shard_transitions),
        Error::NotEmtpyShardTransition{});
}
pub fn verify_empty_shard_transitions<C: Config>(state: &BeaconState<C>,  shard_transitions:Vec<ShardTransition<C>>)->bool{
    
    for shard  in 1..get_active_shard_count(state)
    {

    if state.shard_states[shard as usize].slot != compute_previous_slot(state.slot){ // cia pirmas argumentas yra listas is ShardState kuriame yra slot, kurio ir reikia, ir MAX_SHARDS kurio nereikia jei gerai 
        
        if shard_transitions != ShardTransitions(){ 
            return false
        }

      }
        
    }
    true
}
fn proccess_crosslinks<C: Config>(state: &BeaconState<C>,shard_transitions:Vec<ShardTransition<C>>,attestations: Vec<Attestation<C>>)-> Result<()> {
    
    let on_time_attestation_slot=compute_previous_slot(state.slot);
    let committee_count = get_commitee_count_per_slot(state,compute_epoch_at_slot)
}
#[error("Shard transition in Shard transition processing is not empty")]
NotEmtpyShardTransition{}