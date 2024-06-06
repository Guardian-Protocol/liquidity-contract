#![no_std]
use gmeta::{
    In, 
    InOut,
    Metadata,
};
use gstd::{
    collections::HashMap, 
    prelude::*, 
    ActorId, 
    Decode, 
    Encode, 
    TypeInfo, 
    Vec
};
use state_query::{
    LiquidityQuery, 
    LiquidityResponse
};

pub mod ft_io;
pub mod state_query;
pub mod store_io;

pub type TransactionId = u64;
pub type UnestakeId = u128;
pub type StoreId = usize;

pub type Gvara = u128;
pub type Vara = u128;

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum LiquidStakeAction {
    Stake(u128),
    Unestake {
        amount: Gvara,
        liberation_era: u64,
        liberation_days: u64,
    },
    Withdraw(UnestakeId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum LiquidStakeEvent {
    Success,
    SuccessfullStake,
    SuccessfullUnestake,
    SuccessfullWithdraw(u128),
    StashMessage {
        user: ActorId,
        message_type: String,
        amount: Gvara,
        value: Vara,
    }, 
    TotalLocketBalance {
        total: u128,
    },
    StakeError,
}

#[derive(TypeInfo, Encode, Decode, Debug)]
pub enum LiquidError {
    UserNotFound(String),
    InsuficientBalance(String),
    WithdrawIsNotReady(String),
    UnestakeNotFound(String),
    InternalFTError(String),
    InternalContractError(String),
    InternalStoreError(String),
    StoreNotAvailable(String),
}

#[derive(TypeInfo, Encode, Decode)]
pub struct InitLiquidityCotract {
    pub gvara_contract_address: ActorId,
    pub store_contract_address: Store,
    pub treasure_account: ActorId,
}

#[derive(TypeInfo, Clone, Encode, Decode)]
pub struct Store {
    pub address: ActorId,
    pub is_full: bool,
}

pub struct SecuredInformation {
    pub gvara_token_address: ActorId,
    pub users: HashMap<ActorId, StoreId>,
    pub store_contracts: Vec<Store>,
    pub treasure_account: ActorId,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitLiquidityCotract>;
    type Handle = InOut<LiquidStakeAction, Result<LiquidStakeEvent, LiquidError>>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<LiquidityQuery, Result<LiquidityResponse, LiquidError>>;
}
