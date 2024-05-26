use gstd::{
    ActorId,
    Decode, 
    Encode, 
    String, 
    TypeInfo
};

use crate::{
    UnestakeId, 
    Vara
};

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreAction {
    StoreTransaction {
        user: ActorId,
        transtaction_type: String,
        amount: Vara,
    },
    StoreUnestake {
        user: ActorId,
        amount: Vara,
        liberation_era: u64,
        liberation_days: u64,
    },
    DeleteUnestake(u128),
    FetchUnestake {
        user: ActorId,
        unestake_id: UnestakeId,
    },
    AddAmin(ActorId)
}

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreResponse {
    TransactionStored,
    UnestakeStored(u128),
    UnestakeDeleted,
    Unestake {
        unestake: Unestake,
    },
    AdminAdded,
}

#[derive(TypeInfo, Encode, Decode)]
pub enum StoreError {
    UserNotFound,
    UnestakeNotFound,
    AdminAlreadyExists,
    InssuficientBalance,
    NotAdmin,
    StoreNotAvailable
}

#[derive(TypeInfo, Encode, Decode, Clone)]
pub struct Unestake {
    pub unestake_id: UnestakeId,
    pub amount: Vara,
    pub liberation_era: u64,
    pub liberation_days: u64,
    pub interest_percent: u64
}