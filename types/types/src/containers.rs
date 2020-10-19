//temporary Lighthouse SSZ and hashing implementation
use anyhow::Result;
use bls::{PublicKeyBytes, SignatureBytes};
use serde::{Deserialize, Serialize};
use ssz_new::types::{BitList, BitVector, ByteList, ByteVector, FixedVector, VariableList};
use ssz_new_derive::{SszDecode, SszEncode};
use tree_hash_derive::TreeHash;

use crate::{
    config::Config,
    primitives::{
        AggregateSignatureBytes, CommitteeIndex, DepositIndex, DepositProof, Epoch, Eth1BlockHash,
        Gwei, Shard, Slot, ValidatorIndex, Version, H256,
    },
};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct AggregateAndProof<C: Config> {
    pub aggregator_index: ValidatorIndex,
    pub aggregate: Attestation<C>,
    pub selection_proof: SignatureBytes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct Attestation<C: Config> {
    pub aggregation_bits: BitList<C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub signature: AggregateSignatureBytes,
}

impl<C: Config> Attestation<C> {
    #[must_use]
    pub fn committee_index(&self) -> CommitteeIndex {
        self.data.index
    }

    // REFACTOR(Sifrai Team): Remove? Inline? Rename?
    // DOCUMENT(Sifrai Team): This does not enforce that `self.data == other.data`.
    pub fn aggregate_in_place(&mut self, other: &Self) -> Result<()> {
        self.aggregation_bits = self.aggregation_bits.union(&other.aggregation_bits);
        self.signature.aggregate_in_place(other.signature)?;
        Ok(())
    }
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct AttestationData {
    pub slot: Slot,
    pub index: u64,
    pub beacon_block_root: H256,
    pub source: Checkpoint,
    pub target: Checkpoint,
    // phase1
    pub shard: Shard,
    pub shard_head_root: H256,
    pub shard_transition_root: H256,
}

#[derive(Clone, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct AttesterSlashing<C: Config> {
    pub attestation_1: IndexedAttestation<C>,
    pub attestation_2: IndexedAttestation<C>,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct BeaconBlock<C: Config> {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: H256,
    pub state_root: H256,
    pub body: BeaconBlockBody<C>,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct CustodyChunkChallenge<C: Config> {
    pub responder_index: ValidatorIndex,
    pub shard_transition: ShardTransition<C>,
    pub attestation: Attestation<C>,
    pub data_index: u64,
    pub chunk_index: u64,
}
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct CustodyChunkResponse<C: Config> {
    pub challenge_index: u64,
    pub chunk_index: u64,
    pub chunk: ByteVector<C::BytesPerCustodyChunk>,
    pub branch: FixedVector<H256, C::CustodyResponseDepthInc>,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct CustodyChunkChallengeRecord {
    pub challenge_index: u64,
    pub challenger_index: ValidatorIndex,
    pub responder_index: ValidatorIndex,
    pub inclusion_epoch: Epoch,
    pub data_root: H256,
    pub chunk_index: u64,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct CustodyKeyReveal {
    pub revealer_index: ValidatorIndex,
    pub reveal: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct EarlyDerivedSecretReveal {
    pub revealed_index: ValidatorIndex,
    pub epoch: Epoch,
    pub reveal: SignatureBytes,
    pub masker_index: ValidatorIndex,
    pub mask: H256,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct FullAttestation<C: Config> {
    pub aggregation_bits: BitList<C::MaxValidatorsPerCommittee>,
    pub data: FullAttestationData<C>,
    pub signature: SignatureBytes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct FullAttestationData<C: Config> {
    pub slot: Slot,
    pub index: CommitteeIndex,
    pub beacon_block_root: H256,
    pub source: Checkpoint,
    pub target: Checkpoint,
    pub shard_head_root: H256,
    pub shard_transition: ShardTransition<C>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct SignedCustodySlashing<C: Config> {
    pub message: CustodySlashing<C>,
    pub signature: SignatureBytes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct SignedLightAggregateAndProof<C: Config> {
    pub message: LightAggregateAndProof<C>,
    pub signature: SignatureBytes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct CustodySlashing<C: Config> {
    pub data_index: u64,
    pub malefactor_index: ValidatorIndex,
    pub malefactor_secret: SignatureBytes,
    pub whistleblower_index: ValidatorIndex,
    pub shard_transition: ShardTransition<C>,
    pub attestation: Attestation<C>,
    pub data: ByteList<C::MaxShardBlockSize>,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct ShardState {
    pub slot: Slot,
    pub gasprice: Gwei,
    pub latest_block_root: H256,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct ShardTransition<C: Config> {
    pub start_slot: Slot,
    pub shard_block_lengths: VariableList<u64, C::MaxShardBlocksPerAttestation>,
    pub shard_data_roots: VariableList<H256, C::MaxShardBlocksPerAttestation>,
    pub shard_states: VariableList<ShardState, C::MaxShardBlocksPerAttestation>,
    pub proposer_signature_aggregate: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct CompactCommittee<C: Config> {
    pub pubkeys: VariableList<PublicKeyBytes, C::MaxValidatorsPerCommittee>,
    pub compact_validators: VariableList<u64, C::MaxValidatorsPerCommittee>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct ShardBlock<C: Config> {
    pub shard_parent_root: H256,
    pub beacon_parent_root: H256,
    pub slot: Slot,
    pub shard: Shard,
    pub proposer_index: ValidatorIndex,
    pub body: ByteList<C::MaxShardBlockSize>,
}

impl<C:Config> Default for ShardBlock<C> {
    fn default() -> ShardBlock<C> {
        ShardBlock {
            shard_parent_root: H256::default(),
            beacon_parent_root: H256::default(),
            slot: Slot::default(),
            shard: Shard::default(),
            proposer_index: ValidatorIndex::default(),
            body: ByteList::from_bytes(Vec::new()).unwrap()
        }
    }
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct SignedShardBlock<C: Config> {
    pub message: ShardBlock<C>,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct ShardBlockHeader {
    pub shard_parent_root: H256,
    pub beacon_parent_root: H256,
    pub slot: Slot,
    pub shard: Shard,
    pub proposer_index: ValidatorIndex,
    pub body_root: H256,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct LightClientVote<C: Config> {
    pub data: LightClientVoteData,
    pub aggregation_bits: BitVector<C::LightClientCommitteeSize>,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct LightAggregateAndProof<C: Config> {
    pub aggregator_index: ValidatorIndex,
    pub aggregate: LightClientVote<C>,
    pub selection_proof: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct LightClientVoteData {
    pub slot: Slot,
    pub beacon_block_root: H256,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct BeaconBlockBody<C: Config> {
    pub randao_reveal: SignatureBytes,
    pub eth1_data: Eth1Data,
    pub graffiti: H256,
    pub proposer_slashings: VariableList<ProposerSlashing, C::MaxProposerSlashings>,
    pub attester_slashings: VariableList<AttesterSlashing<C>, C::MaxAttesterSlashings>,
    pub attestations: VariableList<Attestation<C>, C::MaxAttestations>,
    pub deposits: VariableList<Deposit, C::MaxDeposits>,
    pub voluntary_exits: VariableList<SignedVoluntaryExit, C::MaxVoluntaryExits>,
    // phase1
    pub chunk_challenges: VariableList<CustodyChunkChallenge<C>, C::MaxCustodyChunkChallenges>,
    pub chunk_challenge_responses:
        VariableList<CustodyChunkResponse<C>, C::MaxCustodyChunkChallengeResponses>,
    pub custody_key_reveals: VariableList<CustodyKeyReveal, C::MaxCustodyKeyReveals>,
    pub early_derived_secret_reveals:
        VariableList<EarlyDerivedSecretReveal, C::MaxEarlyDerivedSecretReveals>,
    pub custody_slashings: VariableList<SignedCustodySlashing<C>, C::MaxCustodySlashings>,
    pub shard_transitions: FixedVector<ShardTransition<C>, C::MaxShards>,
    pub light_client_bits: BitVector<C::LightClientCommitteeSize>,
    pub light_client_signature: SignatureBytes,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct BeaconBlockHeader {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: H256,
    pub state_root: H256,
    pub body_root: H256,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: H256,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct Deposit {
    pub proof: DepositProof,
    pub data: DepositData,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct DepositData {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: H256,
    pub amount: u64,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct DepositMessage {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: H256,
    pub amount: Gwei,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct Eth1Data {
    pub deposit_root: H256,
    pub deposit_count: DepositIndex,
    pub block_hash: Eth1BlockHash,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct Fork {
    pub previous_version: Version,
    pub current_version: Version,
    pub epoch: Epoch,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct ForkData {
    pub current_version: Version,
    pub genesis_validators_root: H256,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct HistoricalBatch<C: Config> {
    pub block_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
    pub state_roots: FixedVector<H256, C::SlotsPerHistoricalRoot>,
}

#[derive(
    Clone, PartialEq, Eq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct IndexedAttestation<C: Config> {
    pub attesting_indices: VariableList<u64, C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub signature: AggregateSignatureBytes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct PendingAttestation<C: Config> {
    pub aggregation_bits: BitList<C::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub inclusion_delay: u64,
    pub proposer_index: u64,
    // phase1
    pub crosslink_success: bool,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct ProposerSlashing {
    pub signed_header_1: SignedBeaconBlockHeader,
    pub signed_header_2: SignedBeaconBlockHeader,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash)]
pub struct SignedAggregateAndProof<C: Config> {
    pub message: AggregateAndProof<C>,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, PartialEq, Default, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct SignedBeaconBlock<C: Config> {
    pub message: BeaconBlock<C>,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: SignatureBytes,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: SignatureBytes,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct SigningData {
    pub object_root: H256,
    pub domain: H256,
}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Debug,
    Deserialize,
    Serialize,
    SszEncode,
    SszDecode,
    TreeHash,
)]
pub struct Validator {
    pub pubkey: PublicKeyBytes,
    pub withdrawal_credentials: H256,
    pub effective_balance: u64,
    pub slashed: bool,
    pub activation_eligibility_epoch: Epoch,
    pub activation_epoch: Epoch,
    pub exit_epoch: Epoch,
    pub withdrawable_epoch: Epoch,
    // phase1
    pub next_custody_secret_to_reveal: u64,
    pub all_custody_secrets_revealed_epoch: Epoch,
}

#[derive(
    Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize, SszEncode, SszDecode, TreeHash,
)]
pub struct VoluntaryExit {
    pub epoch: Epoch,
    pub validator_index: u64,
}
