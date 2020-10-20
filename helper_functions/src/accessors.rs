use core::{
    cmp::max,
    convert::{TryFrom as _, TryInto as _},
    fmt::Debug,
};

use anyhow::{ensure, Result};
use ethereum_types::H256;
use itertools::Itertools as _;
use ssz_new::types::BitList;
use stubs::beacon_chain;
use typenum::Unsigned as _;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{Attestation, AttestationData, IndexedAttestation},
    primitives::{DomainType, Epoch, Gwei, Shard, Slot, ValidatorIndex, CommitteeIndex},
};

use crate::{cache, error::Error, misc, predicates::is_active_validator};

#[must_use]
pub fn get_current_epoch<C: Config>(state: &BeaconState<C>) -> Epoch {
    misc::compute_epoch_at_slot::<C>(state.slot)
}

#[must_use]
pub fn get_previous_epoch<C: Config>(state: &BeaconState<C>) -> Epoch {
    let current_epoch = get_current_epoch(state);
    if current_epoch == 0 {
        current_epoch
    } else {
        current_epoch - 1
    }
}

pub fn get_block_root<C: Config>(state: &BeaconState<C>, epoch: Epoch) -> Result<H256> {
    get_block_root_at_slot::<C>(state, misc::compute_start_slot_at_epoch::<C>(epoch))
}

pub fn get_block_root_at_slot<C: Config>(state: &BeaconState<C>, slot: Slot) -> Result<H256> {
    ensure!(
        slot < state.slot && state.slot <= slot + C::SlotsPerHistoricalRoot::U64,
        Error::SlotOutOfRange
    );

    state
        .block_roots
        .get(usize::try_from(slot % C::SlotsPerHistoricalRoot::U64)?)
        .copied()
        .ok_or_else(|| Error::IndexOutOfBounds.into())
}

pub fn get_randao_mix<C: Config>(state: &BeaconState<C>, epoch: Epoch) -> Result<H256> {
    state
        .randao_mixes
        .get(usize::try_from(epoch % C::EpochsPerHistoricalVector::U64)?)
        .copied()
        .ok_or_else(|| Error::IndexOutOfBounds.into())
}

pub fn get_active_validator_indices<C: Config>(
    state: &BeaconState<C>,
    epoch: Epoch,
) -> Result<impl Iterator<Item = ValidatorIndex> + '_> {
    let all_indices = 0..state.validators.len().try_into()?;
    let all_validators = state.validators.iter();
    let indices = all_indices
        .zip(all_validators)
        .filter(move |(_, validator)| is_active_validator(**validator, epoch))
        .map(|(index, _)| index);
    Ok(indices)
}

pub fn get_validator_churn_limit<C: Config>(state: &BeaconState<C>) -> Result<u64> {
    let active_validator_count = cache::active_validator_count(state, get_current_epoch(state))?;
    Ok(max(
        C::MIN_PER_EPOCH_CHURN_LIMIT,
        active_validator_count / C::CHURN_LIMIT_QUOTIENT,
    ))
}

pub fn get_seed<C: Config>(
    state: &BeaconState<C>,
    epoch: Epoch,
    domain_type: DomainType,
) -> Result<H256> {
    let mix = get_randao_mix(
        state,
        epoch + C::EpochsPerHistoricalVector::U64 - C::MIN_SEED_LOOKAHEAD - 1,
    )?;

    let mut seed = [0; 44];
    seed[..4].copy_from_slice(&domain_type.to_le_bytes());
    seed[4..12].copy_from_slice(&epoch.to_le_bytes());
    seed[12..].copy_from_slice(mix.as_bytes());

    Ok(hashing::hash(&seed[..]))
}

pub fn get_committee_count_per_slot<C: Config>(
    state: &BeaconState<C>,
    epoch: Epoch,
) -> Result<u64> {
    let active_count = cache::active_validator_count(state, epoch)?
        / C::SlotsPerEpoch::U64
        / C::TARGET_COMMITTEE_SIZE;
    let active_shard_count = beacon_chain::get_active_shard_count::<C>(state);
    let mut count = if active_shard_count < active_count {
        beacon_chain::get_active_shard_count::<C>(state)
    } else {
        active_count
    };

    count = if 1 > count { 1 } else { count };

    Ok(count)
}

pub fn get_beacon_proposer_index<C: Config>(state: &BeaconState<C>) -> Result<ValidatorIndex> {
    get_beacon_proposer_index_at_slot(state, state.slot)
}

pub fn get_beacon_proposer_index_at_slot<C: Config>(
    state: &BeaconState<C>,
    slot: Slot,
) -> Result<ValidatorIndex> {
    let epoch = get_current_epoch(state);
    let seed = get_seed(state, epoch, C::DOMAIN_BEACON_PROPOSER)?;

    let indices = cache::active_validator_indices_ordered(state, epoch)?;

    let mut seed_with_slot = [0; 40];
    seed_with_slot[..32].copy_from_slice(seed.as_bytes());
    seed_with_slot[32..].copy_from_slice(&slot.to_le_bytes());
    let seed = hashing::hash(&seed_with_slot[..]);

    misc::compute_proposer_index(state, &indices, seed)
}

pub fn get_beacon_committee<C: Config>(state: &BeaconState<C>, slot: Slot, index: CommitteeIndex) -> Result<Vec::<ValidatorIndex>> {
    /*
    Return the beacon committee at ``slot`` for ``index``.
    */
    let epoch = misc::compute_epoch_at_slot::<C>(slot);
    let committees_per_slot = get_committee_count_per_slot(state, epoch)?;

    let indices: Vec::<ValidatorIndex> = get_active_validator_indices(state, epoch)?.collect();
    let seed = get_seed(state, epoch, C::DOMAIN_BEACON_ATTESTER)?;
    let index = (slot as u64 % C::SLOTS_PER_EPOCH) * committees_per_slot + index;
    let count = committees_per_slot * C::SLOTS_PER_EPOCH;

    let committee = misc::compute_committee::<C>(indices, seed, index, count)?;
    Ok(committee)
}

pub fn get_total_balance<C: Config>(
    state: &BeaconState<C>,
    indices: impl IntoIterator<Item = ValidatorIndex>,
) -> Result<Gwei> {
    let mut balance: Gwei = 0;
    for validator_index in indices {
        let index = usize::try_from(validator_index)?;
        let validator = state.validators.get(index).ok_or(Error::IndexOutOfBounds)?;
        balance += validator.effective_balance
    }
    Ok(max(C::EFFECTIVE_BALANCE_INCREMENT, balance))
}

#[must_use]
pub fn get_domain<C: Config>(
    state: &BeaconState<C>,
    domain_type: DomainType,
    message_epoch: Option<Epoch>,
) -> H256 {
    let epoch = if message_epoch == None {
        get_current_epoch(state)
    } else {
        message_epoch.expect("Expected a value")
    };
    let fork_version = if epoch < state.fork.epoch {
        state.fork.previous_version
    } else {
        state.fork.current_version
    };
    misc::compute_domain::<C>(
        domain_type,
        Some(&fork_version),
        Some(state.genesis_validators_root),
    )
}

pub fn get_indexed_attestation<C: Config>(
    state: &BeaconState<C>,
    attestation: &Attestation<C>,
) -> Result<IndexedAttestation<C>> {
    let mut attesting_indices =
        get_attesting_indices(state, &attestation.data, &attestation.aggregation_bits)?
            .collect_vec();

    // Sorting a `Vec` is faster than building a `BTreeMap`.
    attesting_indices.sort_unstable();

    let att = IndexedAttestation {
        attesting_indices: attesting_indices.into(),
        data: attestation.data,
        signature: attestation.signature,
    };
    Ok(att)
}

pub fn get_attesting_indices<'aggregation_bits, C: Config>(
    state: &BeaconState<C>,
    attestation_data: &AttestationData,
    aggregation_bits: &'aggregation_bits BitList<C::MaxValidatorsPerCommittee>,
) -> Result<impl Iterator<Item = ValidatorIndex> + Debug + 'aggregation_bits> {
    let committee = cache::beacon_committee(state, attestation_data.slot, attestation_data.index)?;
    Ok(
        // Use indexing to move `committee` into the `impl Iterator` returned from this function.
        // `ArcRef<[_]>` does not implement `IntoIterator` (nor does `Arc<[_]>`).
        (0..committee.len())
            .map(move |index| committee[index])
            .enumerate()
            .filter(move |(position, _)| {
                aggregation_bits
                    .get(*position)
                    .expect("aggregation bitlist length should match committee size")
            })
            .map(|(_, validator_index)| validator_index),
    )
}

pub fn get_committee_count_delta<C: Config>(
    state: &BeaconState<C>,
    start_slot: Slot,
    stop_slot: Slot,
) -> u64 {
    let mut sum: u64 = 0;
    for i in start_slot..stop_slot {
        sum += beacon_chain::get_committee_count_per_slot::<C>(
            state,
            misc::compute_epoch_at_slot::<C>(i),
        );
    }
    sum
}

pub fn get_start_shard<C: Config>(state: &BeaconState<C>, slot: Slot) -> Shard {
    let current_epoch_start_slot =
        misc::compute_start_slot_at_epoch::<C>(get_current_epoch::<C>(state));
    let active_shard_count = beacon_chain::get_active_shard_count::<C>(state);
    match slot {
        slot if slot == current_epoch_start_slot => state.current_epoch_start_shard,
        slot if slot > current_epoch_start_slot => {
            let shard_delta = get_committee_count_delta::<C>(state, current_epoch_start_slot, slot);
            (state.current_epoch_start_shard + shard_delta) % active_shard_count
        }
        _ => {
            let shard_delta = get_committee_count_delta::<C>(state, current_epoch_start_slot, slot);
            let max_committees_per_slot = active_shard_count;
            let max_committees_in_span =
                max_committees_per_slot * (current_epoch_start_slot - slot);
            (state.current_epoch_start_shard + max_committees_in_span - shard_delta)
                % active_shard_count
        }
    }
}

pub fn get_shard_committee<C: Config>(state: &BeaconState<C>, epoch: Epoch, shard: Shard) -> Result<Vec<ValidatorIndex>> {
    let source_epoch = beacon_chain::compute_committee_source_epoch::<C>(epoch, C::SHARD_COMMITTEE_PERIOD);
    
	let active_validator_indices = get_active_validator_indices::<C>(state, epoch);
	let active_validator_indices = match active_validator_indices {
		Ok(active_validator_indices) => {
			let mut active_validator_indices_vec = Vec::new();
			for index in active_validator_indices {
				active_validator_indices_vec.push(index);
			}
			active_validator_indices_vec
		},
		Err(_error) => { 
			let active_validator_indices_vec = Vec::new();
			active_validator_indices_vec
			},
	};
	
    let seed = get_seed(state, source_epoch, 2164260864); // const DOMAIN_SHARD_COMMITTEE is missing so 2164260864 is added
    let seed = match seed {
		Ok(seed) => seed,
		Err(_error) => { panic!() }, // unclear what to return, needs to be fixed
	};
    
    return beacon_chain::compute_committee::<C>(active_validator_indices,
        &seed,
        shard,
        beacon_chain::get_active_shard_count::<C>(state));
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ssz_new::types::FixedVector;
    use typenum::U64;
    use types::{config::MinimalConfig, containers::Validator};

    use super::*;

    #[test]
    fn test_get_current_zero_epoch() {
        let state = BeaconState::<MinimalConfig>::default();
        assert_eq!(get_current_epoch::<MinimalConfig>(&state), 0);
    }

    #[test]
    fn test_get_current_epoch() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 35;
        assert_eq!(get_current_epoch::<MinimalConfig>(&state), 4);
    }

    #[test]
    fn test_get_previous_zero_epoch() {
        let state = BeaconState::<MinimalConfig>::default();
        assert_eq!(get_previous_epoch::<MinimalConfig>(&state), 0);
    }

    #[test]
    fn test_get_previous_epoch() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 35;
        assert_eq!(get_previous_epoch::<MinimalConfig>(&state), 3);
    }

    #[test]
    fn test_get_block_root() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 20;
        // let base: Vec<H256> = vec![H256::from([0; 32]), H256::from([1; 32])];
        let mut base: Vec<H256> = vec![];
        for x in 0..19 {
            base.push(H256::from([x; 32]));
        }
        let roots: FixedVector<_, U64> = FixedVector::from(base);
        state.block_roots = roots;
        let result = get_block_root::<MinimalConfig>(&state, 1);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.expect("Expected success"), H256::from([8; 32]));
    }

    #[test]
    fn test_get_block_root_at_slot() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 2;
        let base: Vec<H256> = vec![H256::from([0; 32]), H256::from([1; 32])];
        let roots: FixedVector<_, U64> = FixedVector::from(base);
        state.block_roots = roots;
        let result = get_block_root_at_slot::<MinimalConfig>(&state, 1);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.expect("Expected success"), H256::from([1; 32]));
    }

    #[test]
    fn test_get_randao_mix() {
        let mut state = BeaconState::<MinimalConfig>::default();
        let base: Vec<H256> = vec![H256::from([0; 32])];
        let mixes: FixedVector<_, U64> = FixedVector::from(base);
        state.randao_mixes = mixes;
        let result = get_randao_mix::<MinimalConfig>(&state, 0);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.expect("Expected success"), H256::from([0; 32]));
    }

    #[test]
    fn test_get_minimal_validator_churn_limit() {
        let state = BeaconState::<MinimalConfig>::default();
        let result = get_validator_churn_limit::<MinimalConfig>(&state);
        assert_eq!(
            result.expect("Expected min_per_epoch_churn_limit"),
            MinimalConfig::MIN_PER_EPOCH_CHURN_LIMIT
        );
    }

    #[test]
    fn test_get_total_minimal_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.validators = Arc::new(vec![Validator::default()].into());
        let result = get_total_balance(&state, core::iter::once(0));
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.expect("Expected success"), 1_000_000_000);
    }

    #[test]
    fn test_get_total_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        let mut validator1 = Validator::default();
        validator1.effective_balance = 17_000_000_000;
        let mut validator2 = Validator::default();
        validator2.effective_balance = 15_000_000_000;
        let mut validator3 = Validator::default();
        validator3.effective_balance = 10_000_000_000;
        state.validators = Arc::new(vec![validator1, validator2, validator3].into());
        let result = get_total_balance(&state, [0, 2].iter().copied());
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.expect("Expected success"), 27_000_000_000);
    }

    #[test]
    fn test_get_active_validators() {
        let mut validator1 = Validator::default();
        validator1.activation_epoch = 0;
        validator1.exit_epoch = 10;
        let mut validator2 = Validator::default();
        validator2.activation_epoch = 0;
        validator2.exit_epoch = 1;
        let mut validator3 = Validator::default();
        validator3.activation_epoch = 0;
        validator3.exit_epoch = 10;

        let mut state = BeaconState::<MinimalConfig>::default();
        state.validators = Arc::new(vec![validator1, validator2, validator3].into());
        let result = get_active_validator_indices::<MinimalConfig>(&state, 3)
            .expect("number of validators should fit in usize");
        assert!(result.eq([0, 2].iter().copied()));
    }
}
