#![no_std]
use core::panic;
use gstd::{exec, msg};

use contract::LiquidStake;
use io::{
    InitLiquidityCotract, 
    SecuredInformation
};

pub mod contract;
pub mod handler;
pub mod ft_contract;
pub mod state;
pub mod store_contract;

static mut LIQUID_STAKE: Option<LiquidStake> = None;
static mut SECURED_INFORMATION: Option<SecuredInformation> = None;

fn liquid_stake_mut() -> &'static mut LiquidStake {
    unsafe { LIQUID_STAKE.get_or_insert(Default::default()) }
}

fn secured_information() -> &'static SecuredInformation {
    unsafe { SECURED_INFORMATION.as_ref().unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitLiquidityCotract = msg::load().expect("Unable to decode message");

    if init_config.gvara_contract_address.is_zero() || init_config.stash_account_address.is_zero() {
        panic!("Invalid address");
    }

    let liquid_stake = LiquidStake {
        owner: msg::source(),
        initial_time: exec::block_timestamp(),
        ..Default::default()
    };

    unsafe {
        SECURED_INFORMATION = Some(SecuredInformation {
            gvara_token_address: init_config.gvara_contract_address.clone(),
            users: HashMap::new(),
            store_contracts: vec![init_config.stash_account_address.clone()],
            treasure_account: init_config.treasure_account.clone(),
        });
    }

    unsafe {
        LIQUID_STAKE = Some(liquid_stake);
    }
}