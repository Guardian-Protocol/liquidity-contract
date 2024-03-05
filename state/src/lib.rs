#![no_std]
use io::*;
use gmeta::metawasm;
use gstd::ActorId;

#[metawasm]
pub mod metafns {

    pub type State = LiquidStakeState;

    pub fn get_locked_balance(state: State, owner: ActorId) -> LiquidStakeEvent {
        let locked = state.users.iter().filter(|&(saved_owner, _)| saved_owner.eq(&owner)).next();

        if locked.is_none() {
            return LiquidStakeEvent::StakeError;
        }

        return LiquidStakeEvent::TotalLocketBalance { total: locked.unwrap().1.user_total_vara_staked };
    }

}