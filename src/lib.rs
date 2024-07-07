#![no_std]
use core::panic;
use gclient::ext::sp_runtime::traits::Clear;
use gstd::{
    collections::HashMap, exec, msg, prog::ProgramGenerator, vec
};

use contract::LiquidStake;
use io::{
    store_io::InitStore, InitLiquidityCotract, SecuredInformation, Store
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

fn secured_information() -> &'static mut SecuredInformation {
    unsafe { SECURED_INFORMATION.as_mut().unwrap_unchecked() }
}

#[no_mangle]
extern "C" fn init() {
    let init_config: InitLiquidityCotract = msg::load().expect("Unable to decode message");

    if init_config.gvara_contract_address.is_zero() || init_config.store_contract_code_id.is_clear() {
        panic!("Invalid address");
    }

    let store_contract_address = ProgramGenerator::create_program(
        init_config.store_contract_code_id, 
        InitStore {
            admins: vec![
                exec::program_id(), 
                msg::source()
            ]
        }, 
        0
    );

    let liquid_stake = LiquidStake {
        owner: msg::source(),
        ..Default::default()
    };

    let store = Store {
        address: store_contract_address.clone(),
        is_full: false
    };

    unsafe {
        SECURED_INFORMATION = Some(SecuredInformation {
            gvara_token_address: init_config.gvara_contract_address.clone(),
            users: HashMap::new(),
            store_contract_code: init_config.store_contract_code_id.clone(),
            store_contracts: vec![store.clone()],
            treasure_account: init_config.treasure_account.clone(),
            stash_account: init_config.stash_account.clone()
        });
    }

    unsafe {
        LIQUID_STAKE = Some(liquid_stake);
    }
}