use gstd::{collections::HashMap, exec, format, msg, ActorId, String, Vec};
use io::{FTAction, FTEvent, LiquidStakeEvent, Tokens, Unestake, UserBalance};

use crate::update_state;

#[derive(Default)]
pub struct LiquidStake {
    pub owner: ActorId,
    pub gvara_token_address: ActorId,
    pub stash_account_address: ActorId,
    pub varatoken_total_staked: Tokens,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: Tokens,
    pub distribution_time: u64,
    pub users: HashMap<ActorId, UserBalance>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: Tokens) {
        if msg::value() != (amount * 1000000000000) {
            panic!("The amount needs be equal to the value sent")
        }

        self.add_liquidity(amount).await;
        self.gvara_transfer_to_user(amount).await;

        update_state();
        
        let _ = msg::send(self.stash_account_address, self.generate_json(amount).await, msg::value());
        let _ = msg::reply(LiquidStakeEvent::SuccessfullStake, 0);
    }

    pub async fn unestake(&mut self, amount: Tokens) {
        let source: ActorId = msg::source().clone();

        if self.users.contains_key(&source) {
            let user_balance = self.users.get(&source).unwrap().clone();

            if user_balance.user_total_vara_staked >= amount {

                self.gvara_transfer_to_contract(amount).await;
                self.remove_liquidity(amount).await;

                update_state();

                let _ = msg::send(self.stash_account_address, self.generate_json(amount).await, 0);
                let _ = msg::reply(LiquidStakeEvent::SuccessfullUnestake, 0);
            } else {
                let _ = msg::reply(LiquidStakeEvent::InsufficientBalance, 0);
            }
        } else {
            let _ = msg::reply(LiquidStakeEvent::UserNotFound, 0);
        }
    }

    async fn add_liquidity(&mut self, amount: Tokens) {
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

    async fn remove_liquidity(&mut self, amount: Tokens) {
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
                            liberation_epoch: 0
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

    async fn gvara_transfer_to_user(&mut self, amount: Tokens) {
        let source: ActorId = msg::source();

        let payload: FTAction = FTAction::Transfer {
            from: exec::program_id(),
            to: source.clone(),
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

    async fn gvara_transfer_to_contract(&mut self, amount: Tokens) {
        let source: ActorId = msg::source();

        let payload: FTAction = FTAction::Transfer { 
            from: source.clone(), 
            to: exec::program_id(), 
            amount: amount.clone() 
        };

        let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
            self.gvara_token_address, 
            payload, 
            0, 0,
        ).expect("Error").await.expect("Unexpected error during sending transfer message");

        match result {
            FTEvent::Ok => { },
            _  => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn generate_json(&mut self, amount: Tokens) -> String {
        return format!("{{
            \"type\": \"stake\",
            \"amount\": {},
            \"source\": \"{:?}\",
            \"value\": {}
        }}", amount, msg::source().clone(), msg::value());
    }
}