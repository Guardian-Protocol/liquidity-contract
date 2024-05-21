use gstd::{async_main, msg};
use io::LiquidStakeAction;

use crate::liquid_stake_mut;

#[async_main]
async fn main() {
    let action = msg::load().expect("Could not load Action");
    let liquid_stake = liquid_stake_mut();

    let result = match action {
        LiquidStakeAction::Stake(amount) => liquid_stake.stake(amount).await,
        LiquidStakeAction::Unestake(amount) => liquid_stake.unestake(amount).await,
        LiquidStakeAction::Withdraw(unestake_id) => liquid_stake.withdraw(unestake_id).await,
    };

    let _ = msg::reply(result, 0);
}