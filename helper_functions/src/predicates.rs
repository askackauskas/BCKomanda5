use core::convert::{TryFrom as _, TryInto as _};

use anyhow::{ensure, Result};
use bit_field::BitField as _;
use itertools::Itertools as _;
use ssz_new::types::VariableList;
use types::{
    beacon_state::BeaconState,
    config::Config,
    consts::FAR_FUTURE_EPOCH,
    containers::{AttestationData, IndexedAttestation, Validator, PendingAttestation},
    primitives::{AggregatePublicKey, AggregateSignature, Epoch, H256, Root, CommitteeIndex},
};

use crate::{accessors, error::Error, misc};

type ValidatorIndexList<C> = VariableList<u64, <C as Config>::MaxValidatorsPerCommittee>;

// Check if validator is active
#[must_use]
pub fn is_active_validator(validator: Validator, epoch: Epoch) -> bool {
    validator.activation_epoch <= epoch && epoch < validator.exit_epoch
}

// Check if `validator` is eligible to be placed into the activation queue.
#[must_use]
pub fn is_eligible_for_activation_queue<C: Config>(validator: Validator) -> bool {
    validator.activation_eligibility_epoch == FAR_FUTURE_EPOCH
        && validator.effective_balance == C::MAX_EFFECTIVE_BALANCE
}

// Check if `validator` is eligible for activation.
#[must_use]
pub fn is_eligible_for_activation<C: Config>(state: &BeaconState<C>, validator: Validator) -> bool {
    // Placement in queue is finalized
    validator.activation_eligibility_epoch <= state.finalized_checkpoint.epoch
        //  Has not yet been activated
        && validator.activation_epoch == FAR_FUTURE_EPOCH
}

// Check if validator is slashable
#[must_use]
pub fn is_slashable_validator(validator: Validator, epoch: Epoch) -> bool {
    !validator.slashed
        && epoch < validator.withdrawable_epoch
        && validator.activation_epoch <= epoch
}

// Check if ``data_1`` and ``data_2`` are slashable according to Casper FFG rules.
#[must_use]
pub fn is_slashable_attestation_data(data_1: &AttestationData, data_2: &AttestationData) -> bool {
    (data_1 != data_2 && data_1.target.epoch == data_2.target.epoch)
        || (data_1.source.epoch < data_2.source.epoch && data_2.target.epoch < data_1.target.epoch)
}

fn aggregate_validator_public_keys<C: Config>(
    indices: &ValidatorIndexList<C>,
    state: &BeaconState<C>,
) -> Result<AggregatePublicKey> {
    let mut aggr_pkey = AggregatePublicKey::default();
    for i in indices.iter() {
        let ind = usize::try_from(*i)?;
        let validator = state.validators.get(ind).ok_or(Error::IndexOutOfBounds)?;
        aggr_pkey.aggregate_in_place(validator.pubkey.try_into()?);
    }
    Ok(aggr_pkey)
}

pub fn validate_indexed_attestation<C: Config>(
    state: &BeaconState<C>,
    indexed_attestation: &IndexedAttestation<C>,
    verify_signature: bool,
) -> Result<()> {
    let indices = &indexed_attestation.attesting_indices;
    ensure!(!indices.is_empty(), Error::AttestingIndicesEmpty);

    let sorted_and_unique = indices.iter().tuple_windows().all(|(a, b)| a < b);
    ensure!(sorted_and_unique, Error::AttestingIndicesNotSortedAndUnique);

    let signature = AggregateSignature::try_from(indexed_attestation.signature)?;

    let aggr_pubkey = aggregate_validator_public_keys(indices, state)?;

    if verify_signature {
        let domain = accessors::get_domain(
            state,
            C::DOMAIN_BEACON_ATTESTER,
            Some(indexed_attestation.data.target.epoch),
        );

        let signing_root = misc::compute_signing_root(&indexed_attestation.data, domain);

        ensure!(
            signature.verify(aggr_pubkey, signing_root.as_bytes()),
            Error::AttestationSignatureInvalid,
        );
    }

    Ok(())
}

pub fn is_valid_merkle_branch(
    leaf: H256,
    branch: &[H256],
    depth: usize,
    index: u64,
    root: H256,
) -> Result<bool> {
    let mut hash = leaf;

    let proof = branch.get(..depth).ok_or(Error::IndexOutOfBounds)?;

    for (height, node) in proof.iter().enumerate() {
        if index.get_bit(height) {
            hash = hashing::concatenate_and_hash(node, hash);
        } else {
            hash = hashing::concatenate_and_hash(hash, node);
        }
    }

    Ok(hash == root)
}

#[must_use]
pub fn is_on_time_attestation<C: Config>(
    state: &BeaconState<C>,
    attestation_data: &AttestationData,
) -> bool {
    misc::compute_previous_slot(state.slot) == attestation_data.slot
}
pub fn is_winning_attestation<C:Config>(state: &BeaconState<C>,attestation: PendingAttestation<C>,committee_index: CommitteeIndex,winning_root: Root) -> bool{

 
is_on_time_attestation(state, &attestation.data) && attestation.data.index == committee_index && attestation.data.shard_transition_root == winning_root

    }

#[cfg(test)]
mod tests {
    use bls::PublicKeyBytes;
    use types::{
        config::MinimalConfig, consts::FAR_FUTURE_EPOCH, containers::Checkpoint, primitives::H256,
    };

    use super::*;

    fn default_validator() -> Validator {
        Validator {
            effective_balance: 0,
            slashed: false,
            activation_eligibility_epoch: FAR_FUTURE_EPOCH,
            activation_epoch: FAR_FUTURE_EPOCH,
            exit_epoch: FAR_FUTURE_EPOCH,
            withdrawable_epoch: FAR_FUTURE_EPOCH,
            withdrawal_credentials: H256([0; 32]),
            pubkey: PublicKeyBytes::default(),
            next_custody_secret_to_reveal: 0,
            all_custody_secrets_revealed_epoch: 0,
        }
    }

    const fn default_attestation_data() -> AttestationData {
        AttestationData {
            beacon_block_root: H256([0; 32]),
            source: Checkpoint {
                epoch: 0,
                root: H256([0; 32]),
            },
            target: Checkpoint {
                epoch: 0,
                root: H256([0; 32]),
            },
            index: 0,
            slot: 0,
            shard: 0,
            shard_head_root: H256([0; 32]),
            shard_transition_root: H256([0; 32]),
        }
    }

    #[test]
    fn test_not_activated() {
        let validator = default_validator();
        let epoch: u64 = 10;

        assert!(!is_active_validator(validator, epoch));
    }

    #[test]
    fn test_is_on_time_attestation_valid() {
        let mut attestation = AttestationData::default();
        attestation.slot = 5;
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 6;

        assert!(is_on_time_attestation(&state, &attestation));
    }

    #[test]
    fn test_is_on_time_attestation_invalid() {
        let mut attestation = AttestationData::default();
        attestation.slot = 5;
        let mut state = BeaconState::<MinimalConfig>::default();
        state.slot = 7;

        assert!(!is_on_time_attestation(&state, &attestation));
    }

    #[test]
    fn test_activated() {
        let mut validator = default_validator();
        validator.activation_epoch = 4;
        let epoch: u64 = 10;

        assert!(is_active_validator(validator, epoch));
    }

    #[test]
    fn test_exited() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.exit_epoch = 10;
        let epoch: u64 = 10;

        assert!(!is_active_validator(validator, epoch));
    }

    #[test]
    fn test_already_slashed() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.slashed = true;
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(validator, epoch));
    }

    #[test]
    fn test_not_slashable_not_active() {
        let validator = default_validator();
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(validator, epoch));
    }

    #[test]
    fn test_not_slashable_withdrawable() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.withdrawable_epoch = 9;
        let epoch: u64 = 10;

        assert!(!is_slashable_validator(validator, epoch));
    }

    #[test]
    fn test_slashable() {
        let mut validator = default_validator();
        validator.activation_epoch = 1;
        validator.withdrawable_epoch = 11;
        let epoch: u64 = 10;

        assert!(is_slashable_validator(validator, epoch));
    }

    #[test]
    fn test_double_vote_attestation_data() {
        let mut data_1 = default_attestation_data();
        let data_2 = default_attestation_data();
        data_1.target.root = H256([1; 32]);

        assert!(is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_equal_attestation_data() {
        let data_1 = default_attestation_data();
        let data_2 = default_attestation_data();

        assert!(!is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_surround_vote_attestation_data() {
        let mut data_1 = default_attestation_data();
        let mut data_2 = default_attestation_data();
        data_1.source.epoch = 0;
        data_2.source.epoch = 1;
        data_1.target.epoch = 4;
        data_2.target.epoch = 3;

        assert!(is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_not_slashable_attestation_data() {
        let mut data_1 = default_attestation_data();
        let mut data_2 = default_attestation_data();
        data_1.source.epoch = 0;
        data_1.target.epoch = 4;
        data_2.source.epoch = 4;
        data_2.target.epoch = 5;
        data_2.source.root = H256([1; 32]);
        data_2.target.root = H256([1; 32]);

        assert!(!is_slashable_attestation_data(&data_1, &data_2));
    }

    #[test]
    fn test_valid_merkle_branch() {
        let leaf_b00 = H256::from([0xAA; 32]);
        let leaf_b01 = H256::from([0xBB; 32]);
        let leaf_b10 = H256::from([0xCC; 32]);
        let leaf_b11 = H256::from([0xDD; 32]);

        let node_b0x = hashing::concatenate_and_hash(leaf_b00, leaf_b01);
        let node_b1x = hashing::concatenate_and_hash(leaf_b10, leaf_b11);

        let root = hashing::concatenate_and_hash(node_b0x, node_b1x);

        assert!(
            is_valid_merkle_branch(leaf_b00, &[leaf_b01, node_b1x], 2, 0, root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(leaf_b01, &[leaf_b00, node_b1x], 2, 1, root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(leaf_b10, &[leaf_b11, node_b0x], 2, 2, root)
                .expect("Unexpected error")
        );

        assert!(
            is_valid_merkle_branch(leaf_b11, &[leaf_b10, node_b0x], 2, 3, root)
                .expect("Unexpected error")
        );
    }

    #[test]
    fn test_merkle_branch_depth() {
        let leaf_b00 = H256::from([0xAF; 32]);
        let leaf_b01 = H256::from([0xBB; 32]);
        let leaf_b10 = H256::from([0xCE; 32]);
        let leaf_b11 = H256::from([0xDB; 32]);

        let node_b0x = hashing::concatenate_and_hash(leaf_b00, leaf_b01);
        let node_b1x = hashing::concatenate_and_hash(leaf_b10, leaf_b11);

        let root = hashing::concatenate_and_hash(node_b0x, node_b1x);

        assert!(
            is_valid_merkle_branch(leaf_b00, &[leaf_b01], 1, 0, node_b0x)
                .expect("Unexpected error")
        );

        assert!(is_valid_merkle_branch(leaf_b00, &[leaf_b01], 3, 0, root).is_err());
    }

    #[test]
    fn test_invalid_merkle_branch() {
        let leaf_b00 = H256::from([0xFF; 32]);
        let leaf_b01 = H256::from([0xAB; 32]);
        let leaf_b10 = H256::from([0xCE; 32]);
        let leaf_b11 = H256::from([0xDB; 32]);

        let node_b0x = hashing::concatenate_and_hash(leaf_b00, leaf_b01);
        let node_b1x = hashing::concatenate_and_hash(leaf_b10, leaf_b11);

        let root = hashing::concatenate_and_hash(node_b0x, node_b1x);

        assert!(!is_valid_merkle_branch(
            leaf_b00,
            &[leaf_b01, node_b0x], // should be node_b1x
            2,
            0,
            root
        )
        .expect("Unexpected error"));

        assert!(!is_valid_merkle_branch(
            leaf_b11,
            &[leaf_b10, node_b0x],
            2,
            3,
            H256::from([0xFF; 32])
        ) // Wrong root
        .expect("Unexpected error"));

        assert!(!is_valid_merkle_branch(
            leaf_b11,
            &[leaf_b10, node_b0x],
            2,
            0, // Wrong index
            root
        )
        .expect("Unexpected error"));
    }

    mod validate_indexed_attestation_tests {
        use std::sync::Arc;

        use bls::{SecretKey, SecretKeyBytes};
        use types::{config::MainnetConfig, primitives::AggregateSignature};

        use super::*;

        #[test]
        fn index_set_not_sorted() {
            let state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(2)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(1)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(3)
                .expect("Unable to add attesting index");

            assert!(validate_indexed_attestation(&state, &attestation, true).is_err());
        }

        #[test]
        fn non_existent_validators() {
            let state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");

            assert!(validate_indexed_attestation(&state, &attestation, true).is_err());
        }

        #[test]
        fn invalid_signature() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(1)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(2)
                .expect("Unable to add attesting index");

            // default_validator() generates randome public key
            state.validators = Arc::new(
                vec![
                    default_validator(),
                    default_validator(),
                    default_validator(),
                ]
                .into(),
            );

            assert!(validate_indexed_attestation(&state, &attestation, true).is_err());
        }

        #[test]
        fn valid_signature() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            let mut attestation: IndexedAttestation<MainnetConfig> = IndexedAttestation::default();
            attestation
                .attesting_indices
                .push(0)
                .expect("Unable to add attesting index");
            attestation
                .attesting_indices
                .push(1)
                .expect("Unable to add attesting index");

            let skey1 =
                SecretKey::try_from(SecretKeyBytes::from(*b"????????????????????????????????"))
                    .expect("bytes represent a valid secret key");
            let v1 = Validator {
                pubkey: skey1.to_public_key().into(),
                ..default_validator()
            };

            let skey2 =
                SecretKey::try_from(SecretKeyBytes::from(*b"!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!"))
                    .expect("bytes represent a valid secret key");
            let v2 = Validator {
                pubkey: skey2.to_public_key().into(),
                ..default_validator()
            };

            state.validators = Arc::new(vec![v1, v2].into());

            attestation.data.beacon_block_root = H256([0xFF; 32]);

            let sig1 = skey1.sign(misc::compute_signing_root(
                &attestation.data,
                accessors::get_domain(
                    &state,
                    MainnetConfig::DOMAIN_BEACON_ATTESTER,
                    Some(attestation.data.target.epoch),
                ),
            ));
            let sig2 = skey2.sign(misc::compute_signing_root(
                &attestation.data,
                accessors::get_domain(
                    &state,
                    MainnetConfig::DOMAIN_BEACON_ATTESTER,
                    Some(attestation.data.target.epoch),
                ),
            ));

            let mut asig = AggregateSignature::default();
            asig.aggregate_in_place(sig1);
            asig.aggregate_in_place(sig2);

            attestation.signature = asig.into();

            let aggr_pubkey =
                aggregate_validator_public_keys(&attestation.attesting_indices, &state)
                    .expect("Success");
            assert!(asig.verify(
                aggr_pubkey,
                misc::compute_signing_root(
                    &attestation.data,
                    accessors::get_domain(
                        &state,
                        MainnetConfig::DOMAIN_BEACON_ATTESTER,
                        Some(attestation.data.target.epoch),
                    ),
                ),
            ));

            assert!(validate_indexed_attestation(&state, &attestation, true).is_ok());
        }
    }
}
