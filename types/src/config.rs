#![allow(clippy::module_name_repetitions)]

use core::{fmt::Debug, num::NonZeroU64, ops::Sub};

use hex_literal::hex;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};
use typenum::{
    IsGreaterOrEqual, Prod, Sub1, Unsigned, B1, U1, U1024, U1048576, U1099511627776, U12, U128,
    U16, U16777216, U2, U2048, U256, U32, U32768, U4, U4096, U64, U65536, U8, U8192, U9,
};

use crate::primitives::{DomainType, Gwei, UnixSeconds, ValidatorIndex, Version};

pub trait Config: Clone + Eq + Default + Debug + Send + Sync + Serialize + 'static {
    type EpochsPerEth1VotingPeriod: Unsigned;
    type EpochsPerSlashingsVector: Eq + Debug + Send + Sync + Unsigned;
    type EpochsPerHistoricalVector: Eq + Debug + Send + Sync + Unsigned;
    type HistoricalRootsLimit: Eq + Debug + Send + Sync + Unsigned;
    type MaxAttesterSlashings: Eq + Debug + Send + Sync + Unsigned;
    type MaxAttestations: Eq + Debug + Send + Sync + Unsigned;
    type MaxDeposits: Eq + Debug + Send + Sync + Unsigned;
    type MaxProposerSlashings: Eq + Debug + Send + Sync + Unsigned;
    type MaxValidatorsPerCommittee: Eq + Debug + Send + Sync + Unsigned;
    type MaxVoluntaryExits: Eq + Debug + Send + Sync + Unsigned;
    type SlotsPerEpoch: Unsigned + Sub<B1>;

    // phase1
    type MaxCustodyChunkChallengeRecords: Eq + Debug + Send + Sync + Unsigned;
    type MaxCustodyKeyReveals: Eq + Debug + Send + Sync + Unsigned;
    type MaxEarlyDerivedSecretReveals: Eq + Debug + Send + Sync + Unsigned;
    type MaxCustodyChunkChallenges: Eq + Debug + Send + Sync + Unsigned;
    type MaxCustodyChunkChallengeResponses: Eq + Debug + Send + Sync + Unsigned;
    type MaxCustodySlashings: Eq + Debug + Send + Sync + Unsigned;
    type MaxShards: Eq + Debug + Send + Sync + Unsigned;
    type MaxShardBlocksPerAttestation: Eq + Debug + Send + Sync + Unsigned;
    type LightClientCommitteeSize: Eq + Debug + Send + Sync + Unsigned;
    type EarlyDerivedSecretPenaltyMaxFutureEpochs: Eq + Debug + Send + Sync + Unsigned;
    type MaxEarlyDerivedSecretRevealsBySlots: Eq + Debug + Send + Sync + Unsigned;
    type BytesPerCustodyChunk: Eq + Debug + Send + Sync + Unsigned;
    type CustodyResponseDepthInc: Eq + Debug + Send + Sync + Unsigned; // TODO properply separate types
    type MaxShardBlockSize: Eq + Debug + Send + Sync + Unsigned;

    // COMMENT(Sifrai Team): `validator::epoch_boundary_block_root` relies on
    //                       `Self::SlotsPerHistoricalRoot: IsGreaterOrEqual<Sub1<Self::SlotsPerEpoch>>`
    //                       which in turn requires `Self::SlotsPerEpoch: Sub<B1>`.
    type SlotsPerHistoricalRoot: Eq
        + Debug
        + Send
        + Sync
        + Unsigned
        + IsGreaterOrEqual<Sub1<Self::SlotsPerEpoch>>;
    type ValidatorRegistryLimit: Eq + Debug + Send + Sync + Unsigned;

    // COMMENT(Sifrai Team): Separate from the others because they're derived.
    type MaxAttestationsPerEpoch: Eq + Debug + Send + Sync + Unsigned;
    type SlotsPerEth1VotingPeriod: Eq + Debug + Send + Sync + Unsigned;

    // COMMENT(Sifrai Team): Separate from the others because it's non-standard.
    // COMMENT(Sifrai Team): `slot_timer::ticks` relies on this being nonzero.
    const THIRD_OF_SLOT: NonZeroU64 = nonzero!(4_u64);

    const BASE_REWARD_FACTOR: u64 = 64;
    const BLS_WITHDRAWAL_PREFIX_BYTE: u8 = 0x00;
    const CHURN_LIMIT_QUOTIENT: u64 = 0x0001_0000;
    const DOMAIN_AGGREGATE_AND_PROOF: DomainType = 6;
    const DOMAIN_BEACON_ATTESTER: DomainType = 1;
    const DOMAIN_BEACON_PROPOSER: DomainType = 0;
    const DOMAIN_DEPOSIT: DomainType = 3;
    const DOMAIN_RANDAO: DomainType = 2;
    const DOMAIN_SELECTION_PROOF: DomainType = 5;
    const DOMAIN_SHARD_PROPOSAL: DomainType = 81;
    const DOMAIN_VOLUNTARY_EXIT: DomainType = 4;
    const EFFECTIVE_BALANCE_INCREMENT: Gwei = 1_000_000_000;
    const EJECTION_BALANCE: Gwei = 16_000_000_000;
    const EPOCHS_PER_RANDOM_SUBNET_SUBSCRIPTION: u64 = 16;
    const ETH1_FOLLOW_DISTANCE: u64 = 1024;
    const GENESIS_DELAY: u64 = 172_800;
    const GENESIS_FORK_VERSION: Version = Version::new(hex!("00000000"));
    const HYSTERESIS_DOWNWARD_MULTIPLIER: u64 = 1;
    const HYSTERESIS_QUOTIENT: u64 = 4;
    const HYSTERESIS_UPWARD_MULTIPLIER: u64 = 5;
    const INACTIVITY_PENALTY_QUOTIENT: u64 = 1 << 24;
    const MAX_COMMITTEES_PER_SLOT: u64 = 64;
    const MAX_EFFECTIVE_BALANCE: Gwei = 32_000_000_000;
    const MAX_SEED_LOOKAHEAD: u64 = 4;
    const MIN_ATTESTATION_INCLUSION_DELAY: u64 = 1;
    const MIN_EPOCHS_TO_INACTIVITY_PENALTY: u64 = 4;
    const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 1 << 14;
    // Bitcoin's 11th anniversary
    // (see <https://github.com/ethereum/eth2.0-specs/issues/1129#issue-448918350>).
    const MIN_GENESIS_TIME: UnixSeconds = 1_578_009_600;
    const MIN_PER_EPOCH_CHURN_LIMIT: u64 = 4;
    const MIN_SEED_LOOKAHEAD: u64 = 1;
    const MIN_SLASHING_PENALTY_QUOTIENT: u64 = 32;
    const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: u64 = 256;
    const PROPOSER_REWARD_QUOTIENT: u64 = 8;
    const RANDOM_SUBNETS_PER_VALIDATOR: u64 = 1;
    const SAFE_SLOTS_TO_UPDATE_JUSTIFIED: u64 = 8;
    const SECONDS_PER_ETH1_BLOCK: u64 = 14;
    const SHARD_COMMITTEE_PERIOD: u64 = 256;
    const SHUFFLE_ROUND_COUNT: u8 = 90;
    const TARGET_AGGREGATORS_PER_COMMITTEE: u64 = 16;
    const TARGET_COMMITTEE_SIZE: u64 = 128;
    const WHISTLEBLOWER_REWARD_QUOTIENT: u64 = 512;

    // COMMENT(Sifrai Team): `slot_timer::next_tick_with_instant` relies on this being nonzero.
    #[must_use]
    fn seconds_per_slot() -> NonZeroU64 {
        // COMMENT(Sifrai Team): Neither `NonZeroU64::new` nor `Option::expect` are currently const.
        NonZeroU64::new(Self::THIRD_OF_SLOT.get() * 3).expect("Config::THIRD_OF_SLOT is NonZeroU64")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Deserialize, Serialize)]
pub struct MainnetConfig {}

impl Config for MainnetConfig {
    type EpochsPerEth1VotingPeriod = U32;
    type EpochsPerSlashingsVector = U8192;
    type EpochsPerHistoricalVector = U65536;
    type HistoricalRootsLimit = U16777216;
    type MaxAttesterSlashings = U2;
    type MaxAttestations = U128;
    type MaxDeposits = U16;
    type MaxProposerSlashings = U16;
    type MaxValidatorsPerCommittee = U2048;
    type MaxVoluntaryExits = U16;
    type SlotsPerEpoch = U32;
    type SlotsPerHistoricalRoot = U8192;
    type ValidatorRegistryLimit = U1099511627776;
    type BytesPerCustodyChunk = U4096;
    type MaxShardBlockSize = U1048576;

    // phase1
    type MaxCustodyChunkChallengeRecords = U1048576;
    type MaxCustodyKeyReveals = U256;
    type MaxEarlyDerivedSecretReveals = U1;
    type MaxCustodyChunkChallenges = U4;
    type MaxCustodyChunkChallengeResponses = U16;
    type MaxCustodySlashings = U1;
    type MaxShards = U1024;
    type MaxShardBlocksPerAttestation = U12;
    type LightClientCommitteeSize = U128;
    type EarlyDerivedSecretPenaltyMaxFutureEpochs = U32768;
    type MaxEarlyDerivedSecretRevealsBySlots = U32;
    type CustodyResponseDepthInc = U9; // ceillog2(MAX_SHARD_BLOCK_SIZE // BYTES_PER_CUSTODY_CHUNK) + 1

    type MaxAttestationsPerEpoch = Prod<Self::MaxAttestations, Self::SlotsPerEpoch>;
    type SlotsPerEth1VotingPeriod = Prod<Self::EpochsPerEth1VotingPeriod, Self::SlotsPerEpoch>;
}

macro_rules! delegate_to_mainnet {
    ($(type $associated_type: ident;)+) => {
        $(type $associated_type = <MainnetConfig as Config>::$associated_type;)+
    };
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Deserialize, Serialize)]
pub struct MinimalConfig;

impl Config for MinimalConfig {
    delegate_to_mainnet! {
        type HistoricalRootsLimit;
        type MaxAttestations;
        type MaxAttesterSlashings;
        type MaxDeposits;
        type MaxProposerSlashings;
        type MaxValidatorsPerCommittee;
        type MaxVoluntaryExits;
        type ValidatorRegistryLimit;

        // phase1
        type MaxCustodyChunkChallengeResponses;
        type MaxCustodyChunkChallengeRecords;
        type MaxCustodyKeyReveals;
        type MaxEarlyDerivedSecretReveals;
        type MaxCustodySlashings;
        type MaxShardBlocksPerAttestation;
        type LightClientCommitteeSize;
        type BytesPerCustodyChunk;
        type CustodyResponseDepthInc;
        type MaxShardBlockSize;
    }

    type EpochsPerEth1VotingPeriod = U4;
    type EpochsPerHistoricalVector = U64;
    type EpochsPerSlashingsVector = U64;
    type SlotsPerEpoch = U8;
    type SlotsPerHistoricalRoot = U64;

    // phase1
    type MaxCustodyChunkChallenges = U2;
    type MaxShards = U8;
    type EarlyDerivedSecretPenaltyMaxFutureEpochs = U64;

    type MaxAttestationsPerEpoch = Prod<Self::MaxAttestations, Self::SlotsPerEpoch>;
    type SlotsPerEth1VotingPeriod = Prod<Self::EpochsPerEth1VotingPeriod, Self::SlotsPerEpoch>;

    // Phase1
    type MaxEarlyDerivedSecretRevealsBySlots = U8;

    const THIRD_OF_SLOT: NonZeroU64 = nonzero!(2_u64);

    const ETH1_FOLLOW_DISTANCE: u64 = 16;
    const GENESIS_DELAY: u64 = 300;
    const GENESIS_FORK_VERSION: Version = Version::new(hex!("00000001"));
    const MAX_COMMITTEES_PER_SLOT: u64 = 4;
    const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: ValidatorIndex = 64;
    const SAFE_SLOTS_TO_UPDATE_JUSTIFIED: u64 = 2;
    const SHARD_COMMITTEE_PERIOD: u64 = 64;
    const SHUFFLE_ROUND_COUNT: u8 = 10;
    const TARGET_COMMITTEE_SIZE: u64 = 4;
}

/// <https://github.com/goerli/medalla/tree/e08f4e6c6f2c8abc327f4375d65414c73177d1dd/medalla>
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug, Deserialize, Serialize)]
pub struct MedallaConfig;

impl Config for MedallaConfig {
    delegate_to_mainnet! {
        type EpochsPerEth1VotingPeriod;
        type EpochsPerSlashingsVector;
        type EpochsPerHistoricalVector;
        type HistoricalRootsLimit;
        type MaxAttesterSlashings;
        type MaxAttestations;
        type MaxDeposits;
        type MaxProposerSlashings;
        type MaxValidatorsPerCommittee;
        type MaxVoluntaryExits;
        type SlotsPerEpoch;
        type SlotsPerHistoricalRoot;
        type ValidatorRegistryLimit;

        type MaxAttestationsPerEpoch;
        type SlotsPerEth1VotingPeriod;

        // phase1
        type MaxCustodyChunkChallengeRecords;
        type MaxCustodyKeyReveals;
        type MaxEarlyDerivedSecretReveals;
        type MaxCustodyChunkChallengeResponses;
        type MaxCustodyChunkChallenges;
        type MaxCustodySlashings;
        type MaxShards;
        type MaxShardBlocksPerAttestation;
        type LightClientCommitteeSize;
        type EarlyDerivedSecretPenaltyMaxFutureEpochs;
        type MaxEarlyDerivedSecretRevealsBySlots;
        type BytesPerCustodyChunk;
        type CustodyResponseDepthInc;
        type MaxShardBlockSize;
    }

    const GENESIS_FORK_VERSION: Version = Version::new(hex!("00000001"));
    const MIN_GENESIS_TIME: UnixSeconds = 1_596_546_008;
}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Debug, Deserialize, Serialize,
)]
pub struct SpadinaConfig;

impl Config for SpadinaConfig {
    delegate_to_mainnet! {
        type EpochsPerEth1VotingPeriod;
        type EpochsPerSlashingsVector;
        type EpochsPerHistoricalVector;
        type HistoricalRootsLimit;
        type MaxAttesterSlashings;
        type MaxAttestations;
        type MaxDeposits;
        type MaxProposerSlashings;
        type MaxValidatorsPerCommittee;
        type MaxVoluntaryExits;
        type SlotsPerEpoch;
        type SlotsPerHistoricalRoot;
        type ValidatorRegistryLimit;

        type MaxAttestationsPerEpoch;
        type SlotsPerEth1VotingPeriod;

        // phase1
        type MaxCustodyChunkChallengeRecords;
        type MaxCustodyKeyReveals;
        type MaxEarlyDerivedSecretReveals;
        type MaxCustodyChunkChallengeResponses;
        type MaxCustodyChunkChallenges;
        type MaxCustodySlashings;
        type MaxShards;
        type MaxShardBlocksPerAttestation;
        type LightClientCommitteeSize;
        type EarlyDerivedSecretPenaltyMaxFutureEpochs;
        type MaxEarlyDerivedSecretRevealsBySlots;
        type BytesPerCustodyChunk;
        type CustodyResponseDepthInc;
        type MaxShardBlockSize;
    }

    const GENESIS_FORK_VERSION: Version = Version::new(hex!("00000002"));
    const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 1024;
    const MIN_GENESIS_TIME: UnixSeconds = 1_601_380_800;
}
