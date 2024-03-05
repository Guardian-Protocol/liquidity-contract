#![no_std]
use core::panic;
use gstd::{exec, msg, ActorId};

use contract::LiquidStake;
use io::LiquidStakeState;

pub mod contract;
pub mod handler;

static mut ADDRESS_FT: Option<ActorId> = None;
static mut LIQUID_STAKE: Option<LiquidStake> = None;
static mut STATE: Option<LiquidStakeState> = None;

fn liquid_stake_mut() -> &'static mut LiquidStake {
    unsafe { LIQUID_STAKE.get_or_insert(Default::default()) }
}

fn state_mut() -> &'static mut LiquidStakeState {
    unsafe { STATE.as_mut().unwrap_unchecked() }
}

fn address_ft_mut() -> &'static mut ActorId {
    unsafe { ADDRESS_FT.as_mut().unwrap_unchecked() }
}

fn update_state() {
    let liquid_stake = liquid_stake_mut();
    let state = state_mut();

    state.owner = liquid_stake.owner;
    state.staking_token_address = liquid_stake.staking_token_address;
    state.varatoken_total_staked = liquid_stake.varatoken_total_staked;
    state.initial_time = liquid_stake.initial_time;
    state.total_time_protocol = liquid_stake.total_time_protocol;
    state.gvaratokens_reward_total = liquid_stake.gvaratokens_reward_total;
    state.distribution_time = liquid_stake.distribution_time;
    state.users = liquid_stake.users.iter().map(|(k, v)| (*k, *v)).collect();
}

#[no_mangle]
extern "C" fn init() {
    let ft_address: ActorId = msg::load().expect("Unable to decode message");

    let liquid_stake = LiquidStake {
        owner: msg::source(),
        initial_time: exec::block_timestamp(),
        ..Default::default()
    };

    if ft_address.is_zero() {
        panic!("Invalid address");
    }

    unsafe {
        ADDRESS_FT = Some(ft_address);
        LIQUID_STAKE = Some(liquid_stake);
    }

    unsafe {
        STATE = Some(LiquidStakeState {
            owner: msg::source(),
            staking_token_address: ft_address,
            initial_time: exec::block_timestamp(),
            ..Default::default()
        });
    }
}

#[no_mangle]
extern "C" fn state() {
    let liquid_stake_state = state_mut();
    let _ = msg::reply(liquid_stake_state, 0);
}