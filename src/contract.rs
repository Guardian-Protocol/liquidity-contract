use gstd::{
    collections::HashMap, 
    exec,
    msg, 
    ActorId, 
    String, 
    Vec
};
use io::{
    Era, 
    Gvara, 
    LiquidStakeEvent, 
    Unestake, 
    UserInformation
};

use crate::update_state;
use crate::secured_information;

use crate::ft_contract::ft_calls;
use crate::utils::json::stash_message;

#[derive(Default)]
pub struct LiquidStake {
    pub owner: ActorId,
    pub gvara_token_address: ActorId,
    pub stash_account_address: ActorId,
    pub varatoken_total_staked: Gvara,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: Gvara,
    pub distribution_time: u64,
    pub users: HashMap<ActorId, UserInformation>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: Gvara) {
        if msg::value() != (amount * 1000000000000) {
            panic!("The amount needs be equal to the value sent")
        }

        self.add_liquidity(amount).await;
        ft_calls::transfer(amount, exec::program_id(), msg::source()).await;

        update_state();

        let stash_message: String = stash_message(amount, String::from("stake")).await;
        
        let _ = msg::send(secured_information().stash_account_address, stash_message, msg::value());
        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    pub async fn unestake(&mut self, amount: Gvara) {
        let user = self.users.get(&msg::source()).expect("User not found");

        if user.user_total_vara_staked < amount {
            panic!("The amount to unestake is greater than the user's balance");
        }

        ft_calls::transfer(amount, msg::source(), exec::program_id()).await;
        self.remove_liquidity(amount).await;

        update_state();

        let stash_message: String = stash_message(amount, String::from("unestake")).await;
        let _ = msg::send(secured_information().stash_account_address, stash_message, 0);

        self.total_time_protocol = exec::block_timestamp() - self.initial_time;
        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    pub async fn update_unestake(&mut self, user: ActorId, era: Era, master_key: ActorId) {

        if master_key != secured_information().master_key {
            panic!("Only the admin account can send this message");
        }

        let user: &mut UserInformation = self.users.get_mut(&user).expect("User not found");

        for (_index, unestake) in user.unestake_history.iter_mut() {
            if unestake.liberation_era == 0 {
                unestake.liberation_era = era;
                unestake.liberation_days = 0;
            }
        }

        update_state();

        self.total_time_protocol = exec::block_timestamp() - self.initial_time;
        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    async fn add_liquidity(&mut self, amount: Gvara) {
        let source: ActorId = msg::source();
        ft_calls::mint(amount).await;

        self.users.entry(source)
            .and_modify(|balance| balance.user_total_vara_staked += amount.clone())
            .or_insert(UserInformation { 
                user_total_vara_staked: amount.clone(), 
                history_id_counter: 0,
                unestake_history: Vec::new()
            }
        );
    }

    async fn remove_liquidity(&mut self, amount: Gvara) {
        let source: ActorId = msg::source();
        ft_calls::burn(amount).await;

        self.users.entry(source)
            .and_modify(|balance| {
                balance.user_total_vara_staked -= amount.clone();
                balance.unestake_history.push((balance.history_id_counter, Unestake {
                    amount: amount.clone(),
                    liberation_era: 0,
                    liberation_days: 0
                }));

                balance.history_id_counter += 1;
            }
        );
    }
}