use gstd::{ActorId, Decode, Encode, TypeInfo};
use crate::Gvara;


#[derive(TypeInfo, Encode, Decode)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: Gvara,
    },
}

#[derive(TypeInfo, Encode, Decode)]
pub enum FTEvent {
    Ok,
    Err,
    Balance(Gvara),
    PermitId(u128),
}

#[derive(TypeInfo)]
pub struct InitFT {
    pub ft_contract_address: ActorId,
}
