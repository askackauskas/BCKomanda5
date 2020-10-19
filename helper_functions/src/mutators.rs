use core::{cmp, convert::TryFrom as _};

use anyhow::Result;
use typenum::Unsigned as _;
use types::{
    arc_ext::ArcExt as _,
    beacon_state::BeaconState,
    config::Config,
    consts::FAR_FUTURE_EPOCH,
    primitives::{Epoch, Gwei, ValidatorIndex},
};

use crate::{
    accessors::{get_current_epoch, get_validator_churn_limit},
    error::Error,
    misc::compute_activation_exit_epoch,
};

use super::accessors;

pub fn increase_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<()> {
    let index = usize::try_from(index)?;
    let balances = state.balances.make_mut();
    let balance = balances.get_mut(index).ok_or(Error::IndexOutOfBounds)?;
    *balance += delta;
    Ok(())
}

pub fn decrease_balance<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
    delta: Gwei,
) -> Result<()> {
    let index = usize::try_from(index)?;
    let balances = state.balances.make_mut();
    let balance = balances.get_mut(index).ok_or(Error::IndexOutOfBounds)?;
    *balance = balance.saturating_sub(delta);
    Ok(())
}

pub fn slash_validator<C: Config>(
    state: &mut BeaconState<C>,
    slashed_index: ValidatorIndex,
    whistleblower_index: Option<ValidatorIndex>,
) -> Result<()> {
    let epoch: Epoch = get_current_epoch(state);
    initiate_validator_exit(state, slashed_index)?;
    let sl_index = usize::try_from(slashed_index)?;
    let validator = &mut state.validators.make_mut()[sl_index];
    validator.slashed = true;
    let epochs_per_slashings = C::EpochsPerSlashingsVector::U64;
    validator.withdrawable_epoch =
        cmp::max(validator.withdrawable_epoch, epoch + epochs_per_slashings);
    let effective_balance = validator.effective_balance;
    let slashings_index = usize::try_from(epoch % epochs_per_slashings)?;
    state.slashings[slashings_index] += effective_balance;
    let decr = validator.effective_balance / C::MIN_SLASHING_PENALTY_QUOTIENT;
    decrease_balance(state, slashed_index, decr)?;

    // Apply proposer and whistleblower rewards
    let proposer_index = accessors::get_beacon_proposer_index(state)?;
    let whistleblower_ind_val = match whistleblower_index {
        None => proposer_index,
        Some(i) => i,
    };
    let whistleblower_reward = effective_balance / C::WHISTLEBLOWER_REWARD_QUOTIENT;
    let proposer_reward = whistleblower_reward / C::PROPOSER_REWARD_QUOTIENT;
    increase_balance(state, proposer_index, proposer_reward)?;
    increase_balance(
        state,
        whistleblower_ind_val,
        whistleblower_reward - proposer_reward,
    )?;
    Ok(())
}

pub fn initiate_validator_exit<C: Config>(
    state: &mut BeaconState<C>,
    index: ValidatorIndex,
) -> Result<()> {
    let validator_index = usize::try_from(index)?;
    let exit_epoch = state.validators[validator_index].exit_epoch;

    if exit_epoch != FAR_FUTURE_EPOCH {
        return Ok(());
    }

    // get exit epochs of all validators
    let validators_number = state.validators.len();
    let mut exit_epochs: Vec<Epoch> = Vec::with_capacity(validators_number);
    for i in 0..validators_number {
        if state.validators[i].exit_epoch != FAR_FUTURE_EPOCH {
            exit_epochs.push(state.validators[i].exit_epoch);
        }
    }

    // get the possible exit epoch - by MIN_SEED_LOOK_AHEAD or the last validator in queue:
    let current_epoch: Epoch = get_current_epoch(state);
    let mut exit_queue_epoch: Epoch = compute_activation_exit_epoch::<C>(current_epoch);
    let iter = exit_epochs.iter();
    for i in iter {
        if *i > exit_queue_epoch {
            exit_queue_epoch = *i;
        }
    }

    // check if number of exiting validators does not exceed churn limit
    let mut exit_queue_churn = 0;
    let iter = exit_epochs.iter();
    for i in iter {
        if *i == exit_queue_epoch {
            exit_queue_churn += 1;
        }
    }
    if exit_queue_churn >= get_validator_churn_limit(state)? {
        exit_queue_epoch += 1;
    }

    // change validator's exit epoch in the beacon chain
    let validator = &mut state.validators.make_mut()[validator_index];
    validator.exit_epoch = exit_queue_epoch;
    validator.withdrawable_epoch = exit_queue_epoch + C::MIN_VALIDATOR_WITHDRAWABILITY_DELAY;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bls::PublicKeyBytes;
    use types::{
        config::{MainnetConfig, MinimalConfig},
        consts::FAR_FUTURE_EPOCH,
        containers::Validator,
        primitives::H256,
    };

    use super::*;

    fn default_validator() -> Validator {
        Validator {
            effective_balance: 0,
            slashed: false,
            activation_eligibility_epoch: FAR_FUTURE_EPOCH,
            activation_epoch: 0,
            exit_epoch: FAR_FUTURE_EPOCH,
            withdrawable_epoch: FAR_FUTURE_EPOCH,
            withdrawal_credentials: H256([0; 32]),
            pubkey: PublicKeyBytes::default(),
            ..types::containers::Validator::default()
        }
    }

    mod slash_validator_tests {
        use super::*;

        #[test]
        fn test_exit_epoch() {
            let mut state: BeaconState<MainnetConfig> = BeaconState::default();
            state.slot = <MainnetConfig as Config>::SlotsPerEpoch::U64 * 3;
            // Add validator and it's balance
            state.validators = Arc::new(vec![default_validator()].into());
            state.balances = Arc::new(vec![100].into());

            let mut state_copy = state.clone();
            initiate_validator_exit(&mut state_copy, 0)
                .expect("Expected successful initiate_validator_exit");

            slash_validator(&mut state, 0, None).expect("slash_validator should succeed");

            assert_eq!(
                state_copy.validators[0].exit_epoch,
                state.validators[0].exit_epoch
            );
        }
    }

    #[test]
    fn test_validator_exit_init() {
        let mut state = BeaconState::<MinimalConfig>::default();

        let val1: Validator = Validator {
            activation_eligibility_epoch: 2,
            activation_epoch: 3,
            effective_balance: 24,
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
        // 1 - exit epoch is already set and should remain the same
        let expected_exit_epoch: Epoch = 4;
        initiate_validator_exit(&mut state, 0).expect("");
        assert_eq!(expected_exit_epoch, state.validators[0].exit_epoch);
        assert_ne!(5, state.validators[0].exit_epoch);
        // 2 - exit epoch is FAR_FUTURE epoch and should be set to the lowest possible value
        initiate_validator_exit(&mut state, 1).expect("");
        assert_ne!(FAR_FUTURE_EPOCH, state.validators[1].exit_epoch);
        assert_eq!(5, state.validators[1].exit_epoch);
    }

    #[test]
    fn test_increase_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.balances = Arc::new(vec![5].into());
        increase_balance(&mut state, 0, 10).expect("");
        assert_eq!(state.balances[0], 15);
    }

    #[test]
    fn test_decrease_balance() {
        let mut state = BeaconState::<MinimalConfig>::default();
        state.balances = Arc::new(vec![5, 10].into());
        decrease_balance(&mut state, 0, 10).expect("");
        assert_eq!(state.balances[0], 0);
        decrease_balance(&mut state, 1, 5).expect("");
        assert_eq!(state.balances[1], 5);
    }
}
