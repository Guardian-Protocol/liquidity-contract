use gstd::{ActorId, Decode, Encode, TypeInfo, Vec};
use crate::Gvara;

pub type TxId = u64;

#[derive(TypeInfo, Encode, Decode)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    TransferToUsers {
        amount: u128,
        to_users: Vec<ActorId>,
    },
    Mint {
        amount: u128,
        to: ActorId,
    },
    Burn {
        amount: u128,
    },
    Transfer {
        tx_id: Option<TxId>,
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        tx_id: Option<TxId>,
        to: ActorId,
        amount: u128,
    },
    BalanceOf(ActorId),
    AddAdmin {
        admin_id: ActorId,
    },
    DeleteAdmin {
        admin_id: ActorId,
    },
}

#[derive(TypeInfo, Encode, Decode, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTEvent {
    Initialized,
    TransferredToUsers {
        from: ActorId,
        to_users: Vec<ActorId>,
        amount: u128,
    },
    Transferred {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approved {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    AdminAdded {
        admin_id: ActorId,
    },
    AdminRemoved {
        admin_id: ActorId,
    },
    Balance(u128),
}

#[derive(TypeInfo, Encode, Decode, Debug)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTError {
    DecimalsError,
    DescriptionError,
    MaxSupplyReached,
    SupplyError,
    NotAdmin,
    NotEnoughBalance,
    ZeroAddress,
    NotAllowedToTransfer,
    AdminAlreadyExists,
    CantDeleteYourself,
    TxAlreadyExists,
}
