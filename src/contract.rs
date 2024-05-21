use gstd::{
    collections::HashMap, exec, format, msg, vec, ActorId, String, ToString, Vec
};
use io::{
    Gvara, LiquidError, LiquidStakeEvent, TransactionHistory, Unestake, UnestakeId, UserInformation 
};

use crate::ft_contract::ft_calls;

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

    pub async fn unestake(&mut self, amount: Gvara) -> Result<LiquidStakeEvent, LiquidError> {
        let user = self.users.get(&msg::source())
            .ok_or_else(|| LiquidError::UserNotFound(format!("User not found {:?}", msg::source()).to_string()));

        if let Err(err) = user {
            return Err(err);
        }

        let user = user.unwrap();
    
        if user.user_total_vara_staked < amount {
            return Err(LiquidError::InsuficientBalance(format!("{} != {}", user.user_total_vara_staked, amount).to_string()));
        }

        if let Err(err) = self.remove_liquidity(&amount).await {
            return Err(err);
        }

        return Ok(LiquidStakeEvent::Success);
    }

    pub async fn withdraw(&mut self, unestake_id: UnestakeId) -> Result<LiquidStakeEvent, LiquidError> {
        let user: &mut UserInformation = self.users.get_mut(&msg::source()).expect("User not found");

        let pos_candidate = user.unestake_history.iter_mut()
            .position(|unestake| unestake.unestake_id == unestake_id)
            .ok_or_else(|| LiquidError::UnestakeNotFound(format!("Unestake not found {:?}", unestake_id).to_string()));

        if let Err(err) = pos_candidate {
            return Err(err);
        }

        let unestake_pos = pos_candidate.unwrap();

        let unestake = user.unestake_history.get(unestake_pos).expect("Unestake not found").clone();
        let unestake_days = (exec::block_timestamp() - unestake.unestake_date_milis) / 86400000;

        if unestake_days < unestake.liberation_days {
            return Err(LiquidError::WithdrawIsNotReady(format!("{} < {}", unestake_days, unestake.liberation_days).to_string()));
        }

        user.unestake_history.remove(unestake_pos);
        return Ok(LiquidStakeEvent::Success);
    }

    async fn add_liquidity(&mut self, amount: &Gvara) -> Result<(), LiquidError> {
        let source: ActorId = msg::source();
        
        let result = ft_calls::mint(*amount).await;

        if let Err(err) = result {
            return Err(err);
        }

        self.users.entry(source)
            .and_modify(|user_information| {
                user_information.user_total_vara_staked += amount.clone();
                user_information.transaction_history.push(
                    TransactionHistory {
                        transaction_id: user_information.history_id_counter,
                        transaction_type: String::from("stake"),
                        transaction_amount: amount.clone(),
                        transaction_time: exec::block_timestamp()
                    }
                );
                
                user_information.history_id_counter += 1;
            })
            .or_insert(UserInformation { 
                user_total_vara_staked: amount.clone(), 
                history_id_counter: 0,
                unestake_id_counter: 0,
                unestake_history: Vec::new(),
                transaction_history: vec![
                    TransactionHistory {
                        transaction_id: 0,
                        transaction_type: String::from("stake"),
                        transaction_amount: amount.clone(),
                        transaction_time: exec::block_timestamp()
                    }
                ]
            }
        );

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

        self.users.entry(source)
            .and_modify(|user_information| {
                user_information.user_total_vara_staked -= amount.clone();

                user_information.unestake_history.push( Unestake {
                    unestake_id: user_information.unestake_id_counter,
                    amount: amount.clone(),
                    liberation_era: 0,
                    liberation_days: 0,
                    unestake_date_milis: exec::block_timestamp()
                });

                user_information.transaction_history.push(
                    TransactionHistory {
                        transaction_id: user_information.history_id_counter,
                        transaction_type: String::from("unestake"),
                        transaction_amount: amount.clone(),
                        transaction_time: exec::block_timestamp()
                    }
                );

                user_information.history_id_counter += 1;
                user_information.unestake_id_counter += 1;
            }
        );

        Ok(())
    }
}