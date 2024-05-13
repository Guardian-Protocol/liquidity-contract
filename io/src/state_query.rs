use gstd::{ActorId, Decode, Encode, TypeInfo, Vec};

use crate::{TransactionHistory, Unestake, Vara};

#[derive(TypeInfo, Encode, Decode)]
pub enum LiquidityQuery {
    GetUserVaraLocked(ActorId),
    GetTransactionHistory(ActorId),
    GetUnestakeHistory(ActorId),
}

#[derive(TypeInfo, Encode, Decode)]
pub enum LiquidityResponse {
    UserVaraLocked(Vara),
    TransactionHistory(Vec<TransactionHistory>),
    UnestakeHistory(Vec<Unestake>),
}