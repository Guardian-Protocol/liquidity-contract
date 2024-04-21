use core::any::Any;

use gstd::ToOwned;
use gstd::{collections::HashMap, exec, format, msg, ActorId, Encode, String, Vec};
use io::{FTAction, FTEvent, Gvara, LiquidStakeAction, LiquidStakeEvent, StashAction, StashEvent, Unestake, UserBalance};

use crate::update_state;
use crate::master_key;

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
    pub users: HashMap<ActorId, UserBalance>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: Gvara) {
        if msg::value() != (amount * 1000000000000) {
            panic!("The amount needs be equal to the value sent")
        }

        self.add_liquidity(amount).await;
        self.transfer_gvara(amount, exec::program_id(), msg::source()).await;

        update_state();

        let stash_message: String = self.generate_json(amount, String::from("stake")).await;
        
        let _ = msg::send(self.stash_account_address, stash_message, msg::value());
        let _ = msg::reply(LiquidStakeEvent::SuccessfullStake, 0);
    }

    pub async fn unestake(&mut self, amount: Gvara) {
        let user = self.users.get(&msg::source()).expect("User not found");

        if user.user_total_vara_staked < amount {
            panic!("The amount to unestake is greater than the user's balance");
        }

        self.transfer_gvara(amount, msg::source(), exec::program_id()).await;
        self.remove_liquidity(amount).await;

        update_state();

        let stash_message: String = self.generate_json(amount, String::from("unestake")).await;
        let _ = msg::send(self.stash_account_address, stash_message, 0);

        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    pub async fn update_unestake(&mut self, user: ActorId, era: u64, days: u32) {
        let account = msg::source();

        // if account != master_key() {
        //     panic!("Only the admin account can call this function");
        // }

        let user = self.users.get_mut(&user).expect("User not found");

        for (_index, unestake) in user.unestake_history.iter_mut() {
            if unestake.liberation_era == 0 {
                unestake.liberation_era = era;
                unestake.liberation_days = days;
            }
        }

        update_state();
        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    async fn add_liquidity(&mut self, amount: Gvara) {
        let source: ActorId = msg::source();

        let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
            self.gvara_token_address, 
            FTAction::Mint(amount.clone()), 
            0, 0
        ).expect("Error").await.expect("Unexpected error during sending mint message");
        
        self.total_time_protocol = exec::block_timestamp() - self.initial_time;

        let _ = match result {
            FTEvent::Ok => {
                self.users.entry(source)
                    .and_modify(|balance| balance.user_total_vara_staked += amount.clone())
                    .or_insert(UserBalance { 
                        user_total_vara_staked: amount.clone(), 
                        history_id_counter: 0,
                        unestake_history: Vec::new()
                    }
                );
            },
            _ => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn remove_liquidity(&mut self, amount: Gvara) {
        let source: ActorId = msg::source();

        let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
            self.gvara_token_address, 
            FTAction::Burn(amount.clone()), 
            0, 0
        ).expect("Error").await.expect("Unexpected error during sending burn message");

        self.total_time_protocol = exec::block_timestamp() - self.initial_time;

        let _ = match result {
            FTEvent::Ok => {
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
            },
            _ => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn transfer_gvara(&mut self, amount: Gvara, from: ActorId, to:ActorId) {
        let payload: FTAction = FTAction::Transfer {
            from: from.clone(),
            to: to.clone(),
            amount: amount.clone(),
        };

        let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
            self.gvara_token_address, 
            payload, 
            0, 0
        ).expect("Error").await.expect("Unexpected error during sending transfer message");

        match result {
            FTEvent::Ok => { },
            _  => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn generate_json(&mut self, amount: Gvara, message_type: String) -> String {
        return format!("{{
            \"type\": \"{}\",
            \"amount\": {},
            \"source\": \"{:?}\",
            \"value\": {}
        }}",message_type, amount, msg::source().clone(), msg::value());
    }
}