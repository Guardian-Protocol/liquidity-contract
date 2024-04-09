use gstd::{collections::HashMap, exec, msg, ActorId};
use io::{FTAction, FTEvent, LiquidStakeAction, LiquidStakeEvent, UserBalance};

use crate::update_state;

#[derive(Default)]
pub struct LiquidStake {
    pub owner: ActorId,
    pub gvara_token_address: ActorId,
    pub stash_account_address: ActorId,
    pub varatoken_total_staked: u128,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: u128,
    pub distribution_time: u64,
    pub users: HashMap<ActorId, UserBalance>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: u128) {
        if msg::value() != amount {
            panic!("The amount needs be equal to the value sent")
        }

        self.add_liquidity(amount).await;
        self.gvara_transfer_to_user(amount).await;

        update_state();
        let _ = msg::send(self.stash_account_address, LiquidStakeAction::Stake(amount), msg::value());
        let _ = msg::reply(LiquidStakeEvent::SuccessfullStake, 0);
    }

    async fn add_liquidity(&mut self, amount_tokens: u128) {
        let source = msg::source();

        let result = msg::send_for_reply_as::<FTAction, FTEvent>(
            self.gvara_token_address, 
            FTAction::Mint(amount_tokens.clone()), 
            0, 0
        ).expect("Error in sending a message").await.expect("message error");
        
        self.total_time_protocol = exec::block_timestamp() - self.initial_time;

        let _ = match result {
            FTEvent::Ok => {
                self.users.entry(source)
                    .and_modify(|balance| balance.user_total_vara_staked += amount_tokens.clone())
                    .or_insert(UserBalance {
                        user_total_vara_staked: amount_tokens.clone(),
                        user_total_gvaratokens: 0,
                });
            },
            _ => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn gvara_transfer_to_user(&mut self, amount_tokens: u128) {
        let source = msg::source();

        let payload = FTAction::Transfer {
            from: exec::program_id(),
            to: source.clone(),
            amount: amount_tokens.clone(),
        };

        let result = msg::send_for_reply_as::<_, FTEvent>(
            self.gvara_token_address, 
            payload, 
            0, 0
        ).expect("Error").await.expect("Error papu");

        match result {
            FTEvent::Ok => {
                self.users.entry(source)
                    .and_modify(|balance| balance.user_total_gvaratokens += amount_tokens.clone());
            },
            _  => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }
}