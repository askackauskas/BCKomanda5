use core::{
    convert::{TryFrom as _, TryInto as _},
    ops::Range,
};

use anyhow::{ensure, Result};
use bit_field::BitArray as _;
use tree_hash::TreeHash;
use typenum::Unsigned as _;
use types::{
    beacon_state::BeaconState,
    config::Config,
    containers::{ForkData, SigningData},
    primitives::{Domain, DomainType, Epoch, ForkDigest, Slot, ValidatorIndex, Version, H256},
};

use crate::error::Error;

#[must_use]
pub fn compute_epoch_at_slot<C: Config>(slot: Slot) -> Epoch {
    slot / C::SlotsPerEpoch::U64
}

#[must_use]
pub fn compute_start_slot_at_epoch<C: Config>(epoch: Epoch) -> Slot {
    epoch * C::SlotsPerEpoch::U64
}

// The specification uses this in at least 2 places:
// - <https://github.com/ethereum/eth2.0-specs/blob/a609320ad4c72cf431cd37219097937721d929a4/specs/phase0/fork-choice.md#compute_slots_since_epoch_start>
// - <https://github.com/ethereum/eth2.0-specs/blob/a609320ad4c72cf431cd37219097937721d929a4/specs/phase0/validator.md#broadcast-attestation>
#[must_use]
pub fn slots_since_epoch_start<C: Config>(slot: Slot) -> u64 {
    slot - compute_start_slot_at_epoch::<C>(compute_epoch_at_slot::<C>(slot))
}

#[must_use]
pub fn slots_in_epoch<C: Config>(epoch: Epoch) -> Range<Slot> {
    compute_start_slot_at_epoch::<C>(epoch)..compute_start_slot_at_epoch::<C>(epoch + 1)
}

#[must_use]
pub fn compute_activation_exit_epoch<C: Config>(epoch: Epoch) -> Epoch {
    epoch + 1 + C::MAX_SEED_LOOKAHEAD
}

#[must_use]
pub fn compute_fork_data_root(current_version: Version, genesis_validators_root: H256) -> H256 {
    ForkData {
        current_version,
        genesis_validators_root,
    }
    .tree_hash_root()
}

#[must_use]
pub fn compute_fork_digest(current_version: Version, genesis_validators_root: H256) -> ForkDigest {
    let root = compute_fork_data_root(current_version, genesis_validators_root);
    ForkDigest::from_slice(&root[..ForkDigest::len_bytes()])
}

#[must_use]
pub fn compute_domain<C: Config>(
    domain_type: DomainType,
    fork_version: Option<&Version>,
    genesis_validators_root: Option<H256>,
) -> Domain {
    let mut domain_bytes = [0; 32];
    let f = fork_version.copied().unwrap_or(C::GENESIS_FORK_VERSION);
    let validators_root = genesis_validators_root.unwrap_or_else(|| H256::from([0; 32]));
    let fork_data_root = compute_fork_data_root(f, validators_root);
    domain_bytes[..4].copy_from_slice(&domain_type.to_le_bytes());
    domain_bytes[4..].copy_from_slice(&fork_data_root[..28]);
    Domain::from(domain_bytes)
}

#[must_use]
pub fn compute_signing_root(object: &impl TreeHash, domain: Domain) -> H256 {
    SigningData {
        object_root: object.tree_hash_root(),
        domain,
    }
    .tree_hash_root()
}

pub fn compute_shuffled_index<C: Config>(
    mut index: ValidatorIndex,
    index_count: u64,
    seed: H256,
) -> Result<ValidatorIndex> {
    ensure!(index < index_count, Error::IndexOutOfBounds);

    for current_round in 0..C::SHUFFLE_ROUND_COUNT {
        let mut buffer = [0; 37];
        buffer[..32].copy_from_slice(seed.as_bytes());
        buffer[32] = current_round;

        let pivot_bytes = hashing::hash(&buffer[..=32])[..core::mem::size_of::<u64>()]
            .try_into()
            .expect("slice has the same size as u64");

        let pivot = u64::from_le_bytes(pivot_bytes) % index_count;

        let flip = (pivot + index_count - index) % index_count;

        let position = index.max(flip);

        buffer[33..].copy_from_slice(&(position / 256).to_le_bytes()[..4]);

        let source = hashing::hash(&buffer[..]);

        let bit_index = (position % 256)
            .try_into()
            .expect("remainder always fits in usize");

        if source.as_bytes().get_bit(bit_index) {
            index = flip;
        }
    }

    Ok(index)
}

pub fn compute_proposer_index<C: Config>(
    state: &BeaconState<C>,
    indices: &[ValidatorIndex],
    seed: H256,
) -> Result<ValidatorIndex> {
    ensure!(!indices.is_empty(), Error::ValidatorIndicesEmpty);
    let max_random_byte = 255;
    let mut i = 0;
    loop {
        let index_count = indices.len().try_into()?;
        let candidate_index = indices[usize::try_from(compute_shuffled_index::<C>(
            i % index_count,
            index_count,
            seed,
        )?)?];
        let rand_bytes = (i / 32).to_le_bytes();
        let mut seed_and_bytes: Vec<u8> = Vec::new();
        for i in 0..32 {
            seed_and_bytes.push(seed[i]);
        }
        let iter = rand_bytes.iter().take(8);
        for i in iter {
            seed_and_bytes.push(*i);
        }
        let hashed_seed_and_bytes = hashing::hash(seed_and_bytes);
        let random_byte = hashed_seed_and_bytes[usize::try_from(i % 32).expect("")];
        let effective_balance =
            state.validators[usize::try_from(candidate_index)?].effective_balance;
        if effective_balance * max_random_byte >= C::MAX_EFFECTIVE_BALANCE * u64::from(random_byte)
        {
            return Ok(candidate_index);
        }
        i += 1;
    }
}

#[must_use]
pub fn compute_previous_slot(slot: Slot) -> Slot {
    if slot > 0 {
        slot - 1
    } else {
        0
    }
}

#[must_use]
pub fn pack_compact_validator(
    index: ValidatorIndex,
    slashed: bool,
    balance_in_increments: u64,
) -> u64 {
    return (index << 16) + ((slashed as u64) << 15) + balance_in_increments;
}

#[must_use]
pub fn compute_committee<C: Config>(
    indices: Vec<ValidatorIndex>,
    seed: H256,
    index: u64,
    count: u64,
) -> Result<Vec<ValidatorIndex>> {
    /*
    Return the committee corresponding to ``indices``, ``seed``, ``index``, and committee ``count``.
    */
    let start = indices.len() as u64 * index / count;
    let end = indices.len() as u64 * (index + 1) as u64 / count;
    let mut committee = Vec::new();
    for i in start..end {
        committee.push(
            indices[compute_shuffled_index::<C>(i as u64, indices.len() as u64, seed)? as usize],
        );
    }
    Ok(committee)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bls::PublicKeyBytes;
    use hex_literal::hex;
    use types::{config::MinimalConfig, consts::FAR_FUTURE_EPOCH, containers::Validator};

    use super::*;

    #[test]
    fn test_epoch_at_slot() {
        assert_eq!(compute_epoch_at_slot::<MinimalConfig>(9), 1);
        assert_eq!(compute_epoch_at_slot::<MinimalConfig>(8), 1);
        assert_eq!(compute_epoch_at_slot::<MinimalConfig>(7), 0);
    }

    #[test]
    fn test_start_slot_at_epoch() {
        assert_eq!(compute_start_slot_at_epoch::<MinimalConfig>(1), 8);
        assert_ne!(compute_start_slot_at_epoch::<MinimalConfig>(1), 7);
        assert_ne!(compute_start_slot_at_epoch::<MinimalConfig>(1), 9);
    }

    #[test]
    fn test_activation_exit_epoch() {
        assert_eq!(compute_activation_exit_epoch::<MinimalConfig>(1), 6);
    }

    #[test]
    fn test_compute_domain() {
        let domain = compute_domain::<MinimalConfig>(1, Some(&hex!("00000001").into()), None);
        assert_eq!(
            domain,
            hex!("0100000018ae4ccbda9538839d79bb18ca09e23e24ae8c1550f56cbb3d84b053").into()
        );
    }
    #[test]
    fn test_compute_shuffled_index() {
        let test_indices_length = 25;
        for _i in 0..20 {
            let shuffled_index: ValidatorIndex =
                compute_shuffled_index::<MinimalConfig>(2, test_indices_length, H256::random())
                    .expect("");
            let in_range = if shuffled_index >= test_indices_length {
                0
            } else {
                1
            };
            // if shuffled index is not one of the validators indices (0, ..., test_indices_length - 1), panic.
            assert_eq!(1, in_range);
        }
    }

    #[test]
    fn test_compute_proposer_index() {
        let mut state = BeaconState::<MinimalConfig>::default();

        let val1: Validator = Validator {
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 0,
            exit_epoch: 4,
            pubkey: PublicKeyBytes::default(),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
            ..types::containers::Validator::default()
        };

        let val2: Validator = Validator {
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
            exit_epoch: FAR_FUTURE_EPOCH,
            pubkey: PublicKeyBytes::default(),
            slashed: false,
            withdrawable_epoch: 9999,
            withdrawal_credentials: H256([0; 32]),
            ..types::containers::Validator::default()
        };

        state.validators = Arc::new(vec![val1, val2].into());
        let index: ValidatorIndex =
            compute_proposer_index(&state, &[0, 1], H256::random()).expect("");
        let in_range = if index >= 2 { 0 } else { 1 };
        assert_eq!(1, in_range);
    }

    #[test]
    fn test_compute_previous_slot() {
        let a: Slot = 5;
        assert_eq!(compute_previous_slot(a), 4);
    }

    #[test]
    fn test_compute_previous_slot_zero() {
        let a: Slot = 0;
        assert_eq!(compute_previous_slot(a), 0);
    }

    #[test]
    fn test_pack_compact_validator() {
        let index: ValidatorIndex = 54321;
        let slashed: bool = true;
        let effective_balance_increment = 12345; // up to 15b

        assert_eq!(
            pack_compact_validator(index, slashed, effective_balance_increment),
            3560026169
        );
    }
}
