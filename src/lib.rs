#![no_std]
use core::panic;
use gstd::{exec, msg};

use contract::LiquidStake;
use io::{InitLiquidityCotract, LiquidStakeState, SecuredInformation};

pub mod contract;
pub mod handler;
pub mod utils;
pub mod ft_contract;

static mut LIQUID_STAKE: Option<LiquidStake> = None;
static mut STATE: Option<LiquidStakeState> = None;
static mut SECURED_INFORMATION: Option<SecuredInformation> = None;

fn liquid_stake_mut() -> &'static mut LiquidStake {
    unsafe { LIQUID_STAKE.get_or_insert(Default::default()) }
}

fn state_mut() -> &'static mut LiquidStakeState {
    unsafe { STATE.as_mut().unwrap_unchecked() }
}

fn secured_information() -> &'static SecuredInformation {
    unsafe { SECURED_INFORMATION.as_ref().unwrap_unchecked() }
}

fn update_state() {
    let liquid_stake = liquid_stake_mut();
    let state = state_mut();

    state.owner = liquid_stake.owner;
    state.gvara_token_address = liquid_stake.gvara_token_address;
    state.varatoken_total_staked = liquid_stake.varatoken_total_staked;
    state.initial_time = liquid_stake.initial_time;
    state.total_time_protocol = liquid_stake.total_time_protocol;
    state.gvaratokens_reward_total = liquid_stake.gvaratokens_reward_total;
    state.distribution_time = liquid_stake.distribution_time;
    state.users = liquid_stake.users.iter().map(|(k, v)| (*k, v.clone())).collect();
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitLiquidityCotract = msg::load().expect("Unable to decode message");

    if init_config.gvara_contract_address.is_zero() || init_config.stash_account_address.is_zero() {
        panic!("Invalid address");
    }

    let liquid_stake = LiquidStake {
        owner: msg::source(),
        gvara_token_address: init_config.gvara_contract_address.clone(),
        stash_account_address: init_config.stash_account_address.clone(),
        initial_time: exec::block_timestamp(),
        ..Default::default()
    };

    unsafe {
        SECURED_INFORMATION = Some(SecuredInformation {
            owner: msg::source(),
            gvara_token_address: init_config.gvara_contract_address.clone(),
            stash_account_address: init_config.stash_account_address.clone(),
            master_key: init_config.master_key.clone(),
        });
    }

    unsafe {
        STATE = Some(LiquidStakeState {
            owner: msg::source(),
            gvara_token_address: init_config.gvara_contract_address.clone(),
            initial_time: exec::block_timestamp(),
            ..Default::default()
        });
    }

    unsafe {
        LIQUID_STAKE = Some(liquid_stake);
    }
}

#[no_mangle]
extern "C" fn state() {
    let liquid_stake_state = state_mut();
    let _ = msg::reply(liquid_stake_state, 0);
}