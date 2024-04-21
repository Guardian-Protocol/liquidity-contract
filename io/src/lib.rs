#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo, Vec, prelude::*};

pub type TransactionId = u64;
pub type Gvara = u128;
pub type Vara = u128;

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum LiquidStakeAction {
    Stake(u128),
    Unestake(u128),
    UpdateUnestake(ActorId, u64, u32),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum LiquidStakeEvent {
    Success,
    SuccessfullStake,
    SuccessfullUnestake,
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

#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum StashAction {
    StashMessage {
        user: ActorId,
        message_type: String,
        amount: Gvara,
        value: Vara,
    },
}

#[derive(Encode, Decode, Clone, TypeInfo)]
pub enum StashEvent {
    UpdateUnestake{
        user: ActorId,
        days: u32,
        era: u64,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}

#[derive(TypeInfo)]
pub struct InitFT {
    pub ft_contract_address: ActorId,
}

#[derive(TypeInfo, Decode, Encode, Clone, Copy)]
pub struct Unestake {
    pub amount: Gvara,
    pub liberation_era: u64,
    pub liberation_days: u32,
}

#[derive(TypeInfo, Decode, Encode, Clone)]
pub struct UserBalance {
    pub user_total_vara_staked: u128,
    pub history_id_counter: u128,
    pub unestake_history: Vec<(u128, Unestake)>
}

#[derive(TypeInfo, Default, Encode, Decode)]
pub struct LiquidStakeState {
    pub owner: ActorId,
    pub gvara_token_address: ActorId,
    pub varatoken_total_staked: u128,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: u128,
    pub distribution_time: u64,
    pub users: Vec<(ActorId, UserBalance)>,
}

#[derive(TypeInfo, Encode, Decode)]
pub struct InitLiquidityCotract {
    pub gvara_contract_address: ActorId,
    pub stash_account_address: ActorId,
    pub master_key: ActorId,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<InitLiquidityCotract>;
    type Handle = InOut<LiquidStakeAction, LiquidStakeEvent>;
    type Others = ();
    type Reply = InOut<String, StashEvent>;
    type Signal = ();
    type State = Out<LiquidStakeState>;
}
