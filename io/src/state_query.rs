use gstd::{
    ActorId, 
    Decode, 
    Encode, 
    TypeInfo
};

#[derive(TypeInfo, Encode, Decode)]
pub enum LiquidityQuery {
    GetUserStore(ActorId),
}

#[derive(TypeInfo, Encode, Decode)]
pub enum LiquidityResponse {
    UserStore(ActorId),
}