use core::convert::{TryFrom as _, TryInto as _};
use std::sync::{Arc, Mutex};

use anyhow::{ensure, Result};
use cached::{once_cell::sync::Lazy, SizedCache};
use owning_ref::ArcRef;
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;
use typenum::Unsigned as _;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{BeaconBlockHeader, Fork},
    primitives::{CommitteeIndex, Epoch, Gwei, Slot, ValidatorIndex, Version, H256},
};

use crate::{
    accessors, cached_ext::CachedExt as _, error::Error, misc, shuffle, weak_key::WeakKey,
};

// Hashing a key of this size may be slow. We could instead store `std::sync::Weak`s pointing to
// `BeaconState`s. However, that would require a rewrite of `beacon_fork_choice::Store`.
//
// Note that this does not contain the `genesis_time` and `genesis_validator_root` fields.
// Their absence could cause collisions between disjoint chains with the same preset,
// but there is currently no way for that to happen.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct BeaconStateKey {
    // This is needed to prevent collisions when a `BeaconState` is processed through empty slots.
    state_slot: Slot,

    // This is needed to prevent collisions between chains with different presets as well as hard
    // forks of the same chain.
    //
    // We rely on the fact that all presets have different values of `genesis_fork_version`.
    // If that were not the case then there would have to be another field that stores a `TypeId`
    // corresponding to the `Config` parameter.
    current_version: Version,

    // `BeaconBlockHeader` fields except for `state_root`.
    latest_block_slot: Slot,
    proposer_index: ValidatorIndex,
    parent_root: H256,
    body_root: H256,
}

impl<C: Config> From<&BeaconState<C>> for BeaconStateKey {
    fn from(state: &BeaconState<C>) -> Self {
        let BeaconState {
            slot: state_slot,
            fork: Fork {
                current_version, ..
            },
            latest_block_header:
                BeaconBlockHeader {
                    slot: latest_block_slot,
                    proposer_index,
                    parent_root,
                    body_root,
                    ..
                },
            ..
        } = *state;

        Self {
            state_slot,
            current_version,
            latest_block_slot,
            proposer_index,
            parent_root,
            body_root,
        }
    }
}

/// [`BeaconState`] with all fields [summarized].
///
/// Treating all fields the same way simplifies the definition of this struct and the implementation
/// of `beacon_state_hash_tree_root`.
///
/// [summarized]:    https://github.com/ethereum/eth2.0-specs/blob/f2440451919d4d7516903ec2646e7a00e12be1b7/ssz/simple-serialize.md#summaries-and-expansions
#[derive(TreeHash)]
struct BeaconStateSummary {
    genesis_time: H256,
    genesis_validators_root: H256,
    slot: H256,
    fork: H256,
    latest_block_header: H256,
    block_roots: H256,
    state_roots: H256,
    historical_roots: H256,
    eth1_data: H256,
    eth1_data_votes: H256,
    eth1_deposit_index: H256,
    validators: H256,
    balances: H256,
    randao_mixes: H256,
    slashings: H256,
    previous_epoch_attestations: H256,
    current_epoch_attestations: H256,
    justification_bits: H256,
    previous_justified_checkpoint: H256,
    current_justified_checkpoint: H256,
    finalized_checkpoint: H256,
}

impl<C: Config> From<&BeaconState<C>> for BeaconStateSummary {
    fn from(state: &BeaconState<C>) -> Self {
        let BeaconState {
            genesis_time,
            genesis_validators_root,
            slot,
            fork,
            latest_block_header,
            block_roots,
            state_roots,
            historical_roots,
            eth1_data,
            eth1_data_votes,
            eth1_deposit_index,
            validators,
            balances,
            randao_mixes,
            slashings,
            previous_epoch_attestations,
            current_epoch_attestations,
            justification_bits,
            previous_justified_checkpoint,
            current_justified_checkpoint,
            finalized_checkpoint,
            ..
        } = state;

        Self {
            genesis_time: genesis_time.tree_hash_root(),
            genesis_validators_root: genesis_validators_root.tree_hash_root(),
            slot: slot.tree_hash_root(),
            fork: fork.tree_hash_root(),
            latest_block_header: latest_block_header.tree_hash_root(),
            block_roots: block_roots.tree_hash_root(),
            state_roots: state_roots.tree_hash_root(),
            historical_roots: historical_roots.tree_hash_root(),
            eth1_data: eth1_data.tree_hash_root(),
            eth1_data_votes: eth1_data_votes.tree_hash_root(),
            eth1_deposit_index: eth1_deposit_index.tree_hash_root(),

            validators: arc_hash_tree_root(validators),
            balances: arc_hash_tree_root(balances),

            randao_mixes: randao_mixes.tree_hash_root(),
            slashings: slashings.tree_hash_root(),
            previous_epoch_attestations: previous_epoch_attestations.tree_hash_root(),
            current_epoch_attestations: current_epoch_attestations.tree_hash_root(),
            justification_bits: justification_bits.tree_hash_root(),
            previous_justified_checkpoint: previous_justified_checkpoint.tree_hash_root(),
            current_justified_checkpoint: current_justified_checkpoint.tree_hash_root(),
            finalized_checkpoint: finalized_checkpoint.tree_hash_root(),
        }
    }
}

#[must_use]
pub fn beacon_state_hash_tree_root(state: &BeaconState<impl Config>) -> H256 {
    BeaconStateSummary::from(state).tree_hash_root()
}

#[must_use]
pub fn arc_hash_tree_root(arc: &Arc<impl TreeHash + Send + Sync + 'static>) -> H256 {
    static CACHE: Lazy<Mutex<SizedCache<WeakKey, H256>>> =
        Lazy::new(|| Mutex::new(SizedCache::with_size(512)));

    let key = arc.into();

    CACHE.look_up(key, || arc.tree_hash_root())
}

pub fn total_active_balance(state: &BeaconState<impl Config>) -> Result<Gwei> {
    thread_local! {
        static CACHE: Mutex<SizedCache<BeaconStateKey, Gwei>> =
            Mutex::new(SizedCache::with_size(256));
    }

    let key = state.into();

    CACHE.try_look_up(key, || {
        let epoch = accessors::get_current_epoch(state);
        let indices = active_validator_indices_unordered(state, epoch)?;
        accessors::get_total_balance(state, indices.iter().copied())
    })
}

pub fn active_validator_count(
    state: &BeaconState<impl Config>,
    epoch: Epoch,
) -> Result<ValidatorIndex> {
    let count = active_validator_indices_unordered(state, epoch)?
        .len()
        .try_into()?;
    Ok(count)
}

pub fn active_validator_indices_unordered(
    state: &BeaconState<impl Config>,
    epoch: Epoch,
) -> Result<Arc<[ValidatorIndex]>> {
    // If `active_validator_indices_ordered` were removed, `active_validator_indices_shuffled` could
    // be used here instead.
    active_validator_indices_ordered(state, epoch)
}

// Only proposer selection needs the list of validators to be in order. Removing this function in
// favor of `accessors::get_active_validator_indices` would save some memory.
pub fn active_validator_indices_ordered(
    state: &BeaconState<impl Config>,
    epoch: Epoch,
) -> Result<Arc<[ValidatorIndex]>> {
    thread_local! {
        #[allow(clippy::type_complexity)]
        static CACHE: Mutex<SizedCache<(BeaconStateKey, Epoch), Arc<[ValidatorIndex]>>> =
            Mutex::new(SizedCache::with_size(256));
    }

    let key = (state.into(), epoch);

    CACHE.try_look_up(key, || {
        let indices = accessors::get_active_validator_indices(state, epoch)?;
        Ok(indices.collect())
    })
}

pub fn beacon_committee<C: Config>(
    state: &BeaconState<C>,
    slot: Slot,
    index_in_slot: CommitteeIndex,
) -> Result<ArcRef<[ValidatorIndex]>> {
    let epoch = misc::compute_epoch_at_slot::<C>(slot);
    let committees_per_slot = accessors::get_committee_count_per_slot::<C>(state, epoch)?;

    ensure!(index_in_slot < committees_per_slot, Error::IndexOutOfBounds);

    let committees_in_epoch = committees_per_slot * C::SlotsPerEpoch::U64;
    let index_in_epoch = (slot % C::SlotsPerEpoch::U64) * committees_per_slot + index_in_slot;
    let indices = active_validator_indices_shuffled(state, epoch)?;

    ArcRef::new(indices).try_map(|indices| {
        let validators = ValidatorIndex::try_from(indices.len())?;
        let start = (validators * index_in_epoch / committees_in_epoch).try_into()?;
        let end = (validators * (index_in_epoch + 1) / committees_in_epoch).try_into()?;
        Ok(&indices[start..end])
    })
}

fn active_validator_indices_shuffled<C: Config>(
    state: &BeaconState<C>,
    epoch: Epoch,
) -> Result<Arc<[ValidatorIndex]>> {
    thread_local! {
        #[allow(clippy::type_complexity)]
        static CACHE: Mutex<SizedCache<(BeaconStateKey, Epoch), Arc<[ValidatorIndex]>>> =
            Mutex::new(SizedCache::with_size(256));
    }

    let key = (state.into(), epoch);

    CACHE.try_look_up(key, || {
        let mut indices = active_validator_indices_ordered(state, epoch)?.to_vec();
        let seed = accessors::get_seed(state, epoch, C::DOMAIN_BEACON_ATTESTER)?;
        shuffle::shuffle::<_, C>(indices.as_mut_slice(), seed);
        Ok(indices.into())
    })
}
