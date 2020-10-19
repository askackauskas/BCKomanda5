use std::vec::Vec;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ShardTransition,ShardBlockHeader},
    primitives::{Shard, Slot, ValidatorIndex},
};

// Stubs for functions yet to be implemented:

fn get_offset_slots<C: Config>(state: BeaconState<C>, shard: Shard) -> Vec<Slot> {
    Vec::new()
}

fn get_shard_proposer_index<C: Config>(beacon_state: BeaconState<C>, slot: &Slot, shard: Shard) -> Validato
}

get_block_root_at_slot(state, offset_slot)

// Shard transition processing:

pub fn apply_shard_transition<C: Config>(state: BeaconState<C>, shard: Shard, transition: ShardTransition<C>) {
    /*// TODO: only need to check it once when phase 1 starts
    assert state.slot > PHASE_1_FORK_SLOT*/

    // Correct data root count
    let offset_slots = get_offset_slots(state, shard);
    /*assert (
        len(transition.shard_data_roots)
        == len(transition.shard_states)
        == len(transition.shard_block_lengths)
        == len(offset_slots)
    )
    assert transition.start_slot == offset_slots[0]*/

    let headers = Vec::new();
    let proposers = Vec::new();
    let mut prev_gasprice = state.shard_states[shard as usize].gasprice;
    let mut shard_parent_root = state.shard_states[shard as usize].latest_block_root;
    for (i, offset_slot) in offset_slots.iter().enumerate() {
        let shard_block_length = transition.shard_block_lengths[i];
        let shard_state = transition.shard_states[i];
        // Verify correct calculation of gas prices and slots
        /*assert shard_state.gasprice == compute_updated_gasprice(prev_gasprice, shard_block_length)
        assert shard_state.slot == offset_slot*/
        // Collect the non-empty proposals result
        match shard_block_length {
            0 => {
                // Must have a stub for `shard_data_root` if empty slot
                /*assert transition.shard_data_roots[i] == Root()*/
            }
            _ => {
                let proposal_index = get_shard_proposer_index(state, offset_slot, shard);
                // Reconstruct shard headers
                let header = ShardBlockHeader(
                    shard_parent_root,
                    beacon_parent_root: get_block_root_at_slot(state, offset_slot),
                    slot=offset_slot,
                    shard=shard,
                    proposer_index=proposal_index,
                    body_root=transition.shard_data_roots[i]
                )
                shard_parent_root = hash_tree_root(header)
                headers.append(header)
                proposers.append(proposal_index)
            }
        }
        prev_gasprice = shard_state.gasprice
    }

    /*let pubkeys = [state.validators[proposer].pubkey for proposer in proposers]
    signing_roots = [
        compute_signing_root(header, get_domain(state, DOMAIN_SHARD_PROPOSAL, compute_epoch_at_slot(header.slot)))
        for header in headers
    ]*/
    /*// Verify combined proposer signature
    assert optional_aggregate_verify(pubkeys, signing_roots, transition.proposer_signature_aggregate)*/

    /*// Copy and save updated shard state
    shard_state = copy(transition.shard_states[len(transition.shard_states) - 1])
    shard_state.slot = compute_previous_slot(state.slot)
    state.shard_states[shard] = shard_state*/
}