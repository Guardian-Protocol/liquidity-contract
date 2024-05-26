use gstd::{async_main, msg, String};
use io::{LiquidError, LiquidStakeAction, LiquidStakeEvent};

use crate::liquid_stake_mut;

#[async_main]
async fn main() {
    let action = msg::load().expect("Could not load Action");
    let liquid_stake = liquid_stake_mut();

    let mut value: u128 = 0;
    let result = match action {
        LiquidStakeAction::Stake(amount) => liquid_stake.stake(amount).await,
        LiquidStakeAction::Unestake { amount, liberation_era, liberation_days } => {
            liquid_stake.unestake(amount, liberation_era, liberation_days).await
        },
        LiquidStakeAction::Withdraw(unestake_id) => {
            if let Ok(LiquidStakeEvent::SuccessfullWithdraw(amount)) = liquid_stake.withdraw(unestake_id).await {
                value = amount;
                Ok(LiquidStakeEvent::SuccessfullWithdraw(amount))
            } else {
                Err(LiquidError::InternalContractError(String::from("Could not withdraw")))
            }
        },
    };

    let _ = msg::reply(result, value);
}