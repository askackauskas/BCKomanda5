use std::sync::Arc;

use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use ssz_new::types::{BitVector, FixedVector, VariableList};
use ssz_new_derive::{SszDecode, SszEncode};

use crate::{
    config::Config,
    consts::JustificationBitsLength,
    containers::{
        BeaconBlockHeader, Checkpoint, CompactCommittee, CustodyChunkChallengeRecord, Eth1Data,
        Fork, PendingAttestation, ShardState, Validator,
    },
    fixed_vector,
    primitives::{Gwei, OnlineEpochs, Shard, Slot, ValidatorIndex, H256},
};

#[cfg(test)]
use tree_hash_derive::TreeHash;

#[derive(Clone, PartialEq, Debug, SmartDefault, Serialize, Deserialize, SszDecode, SszEncode)]
#[cfg_attr(test, derive(TreeHash))]
pub struct BeaconState<C: Config> {
    // Versioning
    pub genesis_time: u64,
    pub genesis_validators_root: H256,
    pub slot: Slot,
    pub fork: Fork,

    // History
    pub latest_block_header: BeaconBlockHeader,
    #[default(fixed_vector::default())]
    pub block_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    #[default(fixed_vector::default())]
    pub state_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    pub historical_roots: VariableList<H256, C::HistoricalRootsLimit>,

    // Eth1
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: VariableList<Eth1Data, C::SlotsPerEth1VotingPeriod>,
    pub eth1_deposit_index: u64,

    // Registry
    pub validators: Arc<VariableList<Validator, C::ValidatorRegistryLimit>>,
    pub balances: Arc<VariableList<Gwei, C::ValidatorRegistryLimit>>,

    // Randomness
    #[default(fixed_vector::default())]
    pub randao_mixes: FixedVector<H256, C::EpochsPerHistoricalVector>,

    // Slashings
    #[default(fixed_vector::default())]
    pub slashings: FixedVector<u64, C::EpochsPerSlashingsVector>,

    // Attestations
    pub previous_epoch_attestations:
        VariableList<PendingAttestation<C>, C::MaxAttestationsPerEpoch>,
    pub current_epoch_attestations: VariableList<PendingAttestation<C>, C::MaxAttestationsPerEpoch>,

    // Finality
    pub justification_bits: BitVector<JustificationBitsLength>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,

    // //Phase1
    pub current_epoch_start_shard: Shard,
    pub shard_states: VariableList<ShardState, C::MaxShards>,
    pub online_countdown: VariableList<OnlineEpochs, C::ValidatorRegistryLimit>,
    pub current_light_committee: CompactCommittee<C>,
    pub next_light_committee: CompactCommittee<C>,
    pub exposed_derived_secrets: FixedVector<
        VariableList<ValidatorIndex, C::MaxEarlyDerivedSecretRevealsBySlots>,
        C::EarlyDerivedSecretPenaltyMaxFutureEpochs,
    >,
    pub custody_chunk_challenge_records:
        VariableList<CustodyChunkChallengeRecord, C::MaxCustodyChunkChallengeRecords>,
    pub custody_chunk_challenge_index: u64,
}
