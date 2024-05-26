use gstd::{
    collections::HashMap, 
    exec, 
    msg,  
    ActorId, 
    String, 
    ToString, 
};

use io::{
    store_io::{
        StoreResponse,
        StoreError
    },
    Gvara, 
    LiquidError, 
    LiquidStakeEvent, 
    Unestake, 
    UnestakeId, 
};

use crate::ft_contract::ft_calls;
use crate::secured_information;
use crate::store_contract::{self, store_calls};

#[derive(Default)]
pub struct LiquidStake {
    pub owner: ActorId,
    pub varatoken_total_staked: Gvara,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub users: HashMap<ActorId, UserInformation>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: Gvara) -> Result<LiquidStakeEvent, LiquidError> {
        if let Err(err) = self.add_liquidity(&amount).await {
            return Err(err);
        }

        if let Err(err) = ft_calls::transfer(amount, msg::source(), exec::program_id()).await {
            return Err(err);
        }

        return Ok(LiquidStakeEvent::Success)
    }

    pub async fn unestake(
        &mut self, 
        amount: Gvara, 
        liberation_era: u64, 
        liberation_days: u64
    ) -> Result<LiquidStakeEvent, LiquidError> {
        if let Ok(StoreResponse::UnestakeStored(id)) = store_calls::store_unestake(
            amount, 
            liberation_era, 
            liberation_days
        ) {
            if let Err(err) = self.remove_liquidity(&amount).await {
                if let Err(_) = store_calls::delete_unestake(id).await {
                    return Err(LiquidError::InternalStoreError("store unavailable".to_string()));
                }
    
                return Err(err);
            }
    
            return Ok(LiquidStakeEvent::Success);   
        } else {
            return Err(LiquidError::InternalStoreError("store unavailable".to_string()));
        }
    }

    pub async fn withdraw(&mut self, unestake_id: UnestakeId) -> Result<LiquidStakeEvent, LiquidError> {
        if let Ok(StoreResponse::Unestake { unestake }) = store_calls::fetch_unestake(unestake_id).await {
            if let Err(err) = store_calls::delete_unestake(unestake_id).await {
                return Err(LiquidError::InternalStoreError("store unavailable".to_string()));
            }

            let total_reawards = exec::value_available();
            let reward = total_reawards * (unestake.interest_percent as u128) / 100;
            let protocol_fee = reward * 10 / 100;
            let user_rewards = reward - protocol_fee;

            let _ = msg::send(secured_information().treasure_account, "rewards", protocol_fee);
            return Ok(LiquidStakeEvent::SuccessfullWithdraw(user_rewards));
        } else {
            return Err(LiquidError::UserNotFound("user not found".to_string()));
        }
    }

    async fn add_liquidity(&mut self, amount: &Gvara) -> Result<(), LiquidError> {
        let source: ActorId = msg::source();

        if let Err(err) = ft_calls::mint(*amount).await {
            return Err(err);
        }

        if let Err(err) = store_calls::store_transaction(transaction_type, amount).await {
            return Err(err);
        }

        Ok(())
    }

    async fn remove_liquidity(&mut self, amount: &Gvara) -> Result<(), LiquidError>  {
        let source: ActorId = msg::source();

        if let Err(err) = ft_calls::transfer(*amount, msg::source(), exec::program_id()).await {
            return Err(err);
        }

        if let Err(err) = ft_calls::burn(*amount).await {
            return Err(err);
        }

        Ok(())
    }
}