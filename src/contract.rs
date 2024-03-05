use gstd::{collections::HashMap, exec, msg, ActorId};
use io::{FTAction, FTEvent, LiquidStakeEvent, UserBalance};

use crate::{address_ft_mut, update_state};

#[derive(Default)]
pub struct LiquidStake {
    pub owner: ActorId,
    pub staking_token_address: ActorId,
    pub varatoken_total_staked: u128,
    pub initial_time: u64,
    pub total_time_protocol: u64,
    pub gvaratokens_reward_total: u128,
    pub distribution_time: u64,
    pub users: HashMap<ActorId, UserBalance>,
}

impl LiquidStake {
    pub async fn stake(&mut self, amount: u128) {
        self.add_liquidity(amount).await;
        self.gvara_transfer_to_user(amount).await;

        update_state();
        let _ = msg::reply(LiquidStakeEvent::SuccessfullStake, 0);
    }

    async fn add_liquidity(&mut self, amount_tokens: u128) {
        let source = msg::source();
        let address_ft = address_ft_mut();

        let result = msg::send_for_reply_as::<_, FTEvent>(
            *address_ft, 
            FTAction::Mint(amount_tokens.clone()), 
            0, 0
        ).expect("Error in sending a message").await;
        
        self.total_time_protocol = exec::block_timestamp() - self.initial_time;

        let _ = match result {
            Ok(FTEvent::Ok) => {
                self.users.entry(source)
                    .and_modify(|balance| balance.user_total_vara_staked += amount_tokens.clone())
                    .or_insert(UserBalance {
                        user_total_vara_staked: amount_tokens.clone(),
                        user_total_gvaratokens: 0,
                    });
            },
            Ok(_) | Err(_)  => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }

    async fn gvara_transfer_to_user(&mut self, amount_tokens: u128) {
        let source = msg::source();
        let address_ft = address_ft_mut();

        let payload = FTAction::Transfer {
            from: exec::program_id(),
            to: source.clone(),
            amount: amount_tokens.clone(),
        };

        let result = msg::send_for_reply_as::<_, FTEvent>(
            *address_ft, 
            payload, 
            0, 0
        ).expect("Error").await;

        match result {
            Ok(FTEvent::Ok) => {
                self.users.entry(source)
                    .and_modify(|balance| balance.user_total_gvaratokens += amount_tokens.clone());
            },
            Ok(_) | Err(_)  => {
                msg::reply(LiquidStakeEvent::StakeError, 0).expect("Error during the reply");
            },
        };
    }
}