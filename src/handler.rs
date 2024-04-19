use gstd::{async_main, msg};
use io::LiquidStakeAction;

use crate::liquid_stake_mut;

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
    };
}