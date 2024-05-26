use gstd::{
    format, 
    msg,  
    String
};
use io::{
    state_query::{
        LiquidityQuery, 
        LiquidityResponse
    }, 
    LiquidError
};

use crate::{
    contract::LiquidStake, 
    secured_information
};

#[no_mangle]
extern "C" fn state() {
    let query: LiquidityQuery = msg::load().expect("Unable to decode message");
    let sec_information = secured_information();

    let result = match query {
        LiquidityQuery::GetUserStore(actor_id) => {
            if let Some(store_id) = sec_information.users.get(&actor_id) {
                Ok(LiquidityResponse::UserStore(sec_information.store_contracts.get(store_id).unwrap().clone()))
            } else {
                Err(LiquidError::StoreNotAvailable)
            }
        }
    };

    let _ = msg::reply(result, 0);
}