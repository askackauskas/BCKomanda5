pub mod arc_ext;
pub mod beacon_state;
pub mod config;
pub mod consts;
pub mod containers;
pub mod fixed_vector;
pub mod primitives;

#[cfg(test)]
mod spec_tests {
    use core::fmt::Debug;

    use serde::{de::DeserializeOwned, Deserialize};
    use spec_test_utils::Case;
    use ssz_new::{SszDecode, SszEncode};
    use test_generator::test_resources;
    use tree_hash::TreeHash;

    use crate::{
        config::{MainnetConfig, MinimalConfig},
        primitives::H256,
    };

    mod tested_types {
        pub use crate::{beacon_state::BeaconState, containers::*};
    }

    #[derive(Deserialize)]
    struct Roots {
        root: H256,
    }

    macro_rules! tests_for_type {
        (
            $type: ident $(<_ $bracket: tt)?,
            $mainnet_glob: literal,
            $minimal_glob: literal,
        ) => {
            #[allow(non_snake_case)]
            mod $type {
                use super::*;

                #[test_resources($mainnet_glob)]
                fn mainnet(case: Case) {
                    run_case::<tested_types::$type$(<MainnetConfig $bracket)?>(case);
                }

                #[test_resources($minimal_glob)]
                fn minimal(case: Case) {
                    run_case::<tested_types::$type$(<MinimalConfig $bracket)?>(case);
                }
            }
        };
    }

    // `Eth1Block` as defined in the specification is meant as an example (an "abstract object") and
    // only contains one field. For whatever reason there are tests for it anyway. We ignore them.

    tests_for_type! {
        AggregateAndProof<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/AggregateAndProof/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/AggregateAndProof/*/*",
    }

    tests_for_type! {
        Attestation<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Attestation/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Attestation/*/*",
    }

    tests_for_type! {
        AttestationData,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/AttestationData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/AttestationData/*/*",
    }

    tests_for_type! {
        AttesterSlashing<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/AttesterSlashing/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/AttesterSlashing/*/*",
    }

    tests_for_type! {
        BeaconBlock<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/BeaconBlock/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/BeaconBlock/*/*",
    }

    tests_for_type! {
        BeaconBlockBody<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/BeaconBlockBody/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/BeaconBlockBody/*/*",
    }

    tests_for_type! {
        BeaconBlockHeader,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/BeaconBlockHeader/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/BeaconBlockHeader/*/*",
    }

    tests_for_type! {
        BeaconState<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/BeaconState/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/BeaconState/*/*",
    }

    tests_for_type! {
        Checkpoint,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Checkpoint/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Checkpoint/*/*",
    }

    tests_for_type! {
        CompactCommittee<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CompactCommittee/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CompactCommittee/*/*",
    }

    tests_for_type! {
        CustodyChunkChallenge<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CustodyChunkChallenge/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CustodyChunkChallenge/*/*",
    }

    tests_for_type! {
        CustodyChunkChallengeRecord,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CustodyChunkChallengeRecord/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CustodyChunkChallengeRecord/*/*",
    }

    tests_for_type! {
        CustodyChunkResponse<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CustodyChunkResponse/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CustodyChunkResponse/*/*",
    }

    tests_for_type! {
        CustodyKeyReveal,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CustodyKeyReveal/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CustodyKeyReveal/*/*",
    }

    tests_for_type! {
        CustodySlashing<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/CustodySlashing/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/CustodySlashing/*/*",
    }

    tests_for_type! {
        Deposit,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Deposit/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Deposit/*/*",
    }

    tests_for_type! {
        DepositData,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/DepositData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/DepositData/*/*",
    }

    tests_for_type! {
        DepositMessage,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/DepositMessage/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/DepositMessage/*/*",
    }

    tests_for_type! {
        EarlyDerivedSecretReveal,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/EarlyDerivedSecretReveal/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/EarlyDerivedSecretReveal/*/*",
    }

    tests_for_type! {
        Eth1Data,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Eth1Data/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Eth1Data/*/*",
    }

    tests_for_type! {
        Fork,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Fork/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Fork/*/*",
    }

    tests_for_type! {
        ForkData,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ForkData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ForkData/*/*",
    }

    tests_for_type! {
        FullAttestation<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/FullAttestation/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/FullAttestation/*/*",
    }

    tests_for_type! {
        FullAttestationData<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/FullAttestationData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/FullAttestationData/*/*",
    }

    tests_for_type! {
        HistoricalBatch<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/HistoricalBatch/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/HistoricalBatch/*/*",
    }

    tests_for_type! {
        IndexedAttestation<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/IndexedAttestation/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/IndexedAttestation/*/*",
    }

    tests_for_type! {
        LightAggregateAndProof<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/LightAggregateAndProof/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/LightAggregateAndProof/*/*",
    }

    tests_for_type! {
        LightClientVote<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/LightClientVote/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/LightClientVote/*/*",
    }

    tests_for_type! {
        LightClientVoteData,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/LightClientVoteData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/LightClientVoteData/*/*",
    }

    tests_for_type! {
        PendingAttestation<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/PendingAttestation/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/PendingAttestation/*/*",
    }

    tests_for_type! {
        ProposerSlashing,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ProposerSlashing/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ProposerSlashing/*/*",
    }

    tests_for_type! {
        ShardBlock<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ShardBlock/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ShardBlock/*/*",
    }

    tests_for_type! {
        ShardBlockHeader,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ShardBlockHeader/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ShardBlockHeader/*/*",
    }

    tests_for_type! {
        ShardState,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ShardState/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ShardState/*/*",
    }

    tests_for_type! {
        ShardTransition<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/ShardTransition/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/ShardTransition/*/*",
    }

    tests_for_type! {
        SignedAggregateAndProof<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedAggregateAndProof/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedAggregateAndProof/*/*",
    }

    tests_for_type! {
        SignedBeaconBlock<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedBeaconBlock/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedBeaconBlock/*/*",
    }

    tests_for_type! {
        SignedBeaconBlockHeader,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedBeaconBlockHeader/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedBeaconBlockHeader/*/*",
    }

    tests_for_type! {
        SignedCustodySlashing<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedCustodySlashing/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedCustodySlashing/*/*",
    }

    tests_for_type! {
        SignedLightAggregateAndProof<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedLightAggregateAndProof/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedLightAggregateAndProof/*/*",
    }

    tests_for_type! {
        SignedShardBlock<_>,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedShardBlock/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedShardBlock/*/*",
    }

    tests_for_type! {
        SignedVoluntaryExit,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SignedVoluntaryExit/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SignedVoluntaryExit/*/*",
    }

    tests_for_type! {
        SigningData,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/SigningData/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/SigningData/*/*",
    }

    tests_for_type! {
        Validator,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/Validator/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/Validator/*/*",
    }

    tests_for_type! {
        VoluntaryExit,
        "eth2.0-spec-tests/tests/mainnet/phase1/ssz_static/VoluntaryExit/*/*",
        "eth2.0-spec-tests/tests/minimal/phase1/ssz_static/VoluntaryExit/*/*",
    }

    fn run_case<D>(case: Case)
    where
        D: PartialEq + Debug + DeserializeOwned + SszDecode + SszEncode + TreeHash,
    {
        let ssz_bytes = case.bytes("serialized.ssz");
        let yaml_value = case.yaml("value");
        let Roots { root } = case.yaml("roots");

        let ssz_value = D::from_ssz_bytes(ssz_bytes.as_slice())
            .expect("the file should contain a value encoded in SSZ");

        assert_eq!(ssz_value, yaml_value);
        assert_eq!(ssz_bytes, yaml_value.as_ssz_bytes());
        assert_eq!(yaml_value.tree_hash_root(), root);
    }
}
