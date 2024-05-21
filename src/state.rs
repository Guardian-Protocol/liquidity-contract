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
    liquid_stake_mut
};

#[no_mangle]
extern "C" fn state() {
    let query: LiquidityQuery = msg::load().expect("Unable to decode message");
    let liquid_stake: &mut LiquidStake = liquid_stake_mut();

    let result = match query {
        LiquidityQuery::GetUserVaraLocked(source) => {
            if let Some(user) = liquid_stake.users.get(&source) {
                let locked_balance = user.user_total_vara_staked;
                Ok(LiquidityResponse::UserVaraLocked(locked_balance.clone()))
            } else {
                Err(LiquidError::UserNotFound(String::from(format!("User not found {:?}", &source))))
            }
        }
        LiquidityQuery::GetTransactionHistory(source) => {
            if let Some(user) = liquid_stake.users.get(&source) {
                let transaction_history = user.transaction_history.clone();
                Ok(LiquidityResponse::TransactionHistory(transaction_history.clone()))
            } else {
                Err(LiquidError::UserNotFound(String::from(format!("User not found {:?}", &source))))
            }
        },
        LiquidityQuery::GetUnestakeHistory(source) => {
            if let Some(user) = liquid_stake.users.get(&source) {
                let unestake_history = user.unestake_history.clone();
                Ok(LiquidityResponse::UnestakeHistory(unestake_history.clone()))
            } else {
                Err(LiquidError::UserNotFound(String::from(format!("User not found {:?}", &source))))
            }
        }
    };

    let _ = msg::reply(result, 0);
}