#![no_std]
use gmeta::{In, InOut, Metadata, Out};
use gstd::{ActorId, Decode, Encode, TypeInfo, Vec, prelude::*};

pub type TransactionId = u64;

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum LiquidStakeAction {
    Stake(u128),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum LiquidStakeEvent {
    SuccessfullStake,
    SuccessfullUnstake,
    TotalLocketBalance {
        total: u128,
    },
    StakeError,
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

#[derive(TypeInfo, Decode, Encode, Copy, Clone)]
pub struct UserBalance {
    pub user_total_vara_staked: u128,
    pub user_total_gvaratokens: u128,
}

#[derive(TypeInfo, Default, Encode, Decode)]
pub struct LiquidStakeState {
    pub owner: ActorId,
    pub staking_token_address: ActorId,
    pub varatoken_total_staked: u128,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: u128,
    pub distribution_time: u64,
    pub users: Vec<(ActorId, UserBalance)>,
}

pub struct ContractMetadata;

impl Metadata for ContractMetadata {
    type Init = In<ActorId>;
    type Handle = InOut<LiquidStakeAction, LiquidStakeEvent>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = Out<LiquidStakeState>;
}
