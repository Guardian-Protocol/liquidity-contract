use gstd::{
    collections::HashMap, 
    exec,
    msg, 
    ActorId, 
    String, 
    Vec,
    vec
};
use io::{
    Gvara, 
    LiquidStakeEvent, 
    TransactionHistory, 
    Unestake, 
    UnestakeId, 
    UserInformation, 
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
    pub async fn stake(&mut self, amount: Gvara) {
        if msg::value() != (amount * 1000000000000) {
            panic!("The amount needs be equal to the value sent")
        }

        self.add_liquidity(&amount).await;
        ft_calls::transfer(amount, exec::program_id(), msg::source()).await;

        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    pub async fn unestake(&mut self, amount: Gvara) {
        let user = self.users.get(&msg::source()).expect("User not found");

        if user.user_total_vara_staked < amount {
            panic!("The amount to unestake is greater than the user's user_information");
        }

        ft_calls::transfer(amount, msg::source(), exec::program_id()).await;
        self.remove_liquidity(&amount).await;

        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    pub async fn withdraw(&mut self, unestake_id: UnestakeId) {
        let user: &mut UserInformation = self.users.get_mut(&msg::source()).expect("User not found");

        let unestake_pos = user.unestake_history.iter_mut()
            .position(|unestake| unestake.unestake_id == unestake_id)
            .expect("Unestake not found");

        let unestake = user.unestake_history.get(unestake_pos).expect("Unestake not found").clone();
        let unestake_days = (exec::block_timestamp() - unestake.unestake_date_milis) / 86400000;

        if unestake_days < unestake.liberation_days {
            panic!("The unestake is not yet available for withdrawal left days: {}", unestake_days);
        }

        user.unestake_history.remove(unestake_pos);
        let _ = msg::reply(LiquidStakeEvent::Success, 0);
    }

    async fn add_liquidity(&mut self, amount: &Gvara) {
        let source: ActorId = msg::source();
        ft_calls::mint(*amount).await;

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
    }

    async fn remove_liquidity(&mut self, amount: &Gvara) {
        let source: ActorId = msg::source();
        ft_calls::burn(*amount).await;

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
    }
}