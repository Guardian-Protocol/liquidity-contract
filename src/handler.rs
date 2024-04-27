use gstd::{async_main, msg};
use io::LiquidStakeAction;

use crate::liquid_stake_mut;
use crate::update_state;

#[async_main]
async fn main() {
    let action = msg::load().expect("Could not load Action");
    let liquid_stake = liquid_stake_mut();

    match action {
        LiquidStakeAction::Stake(amount) => {
            liquid_stake.stake(amount).await;
        }
        
        LiquidStakeAction::Unestake(amount) => {
            liquid_stake.unestake(amount).await;
        }

        LiquidStakeAction::Withdraw(amount) => {
            liquid_stake.withdraw(amount).await;
        }

        LiquidStakeAction::UpdateUnestake { user, liberation_era, liberation_days } => {
            liquid_stake.update_unestake(user, liberation_era, liberation_days).await;
        }

        LiquidStakeAction::CompleteWithdraw { user } => {
            liquid_stake.complete_withdraw(user).await;
        }
    };

    update_state();
}