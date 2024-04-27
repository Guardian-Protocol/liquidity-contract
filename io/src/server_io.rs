use crate::Vara;


#[derive(Debug, Clone)]
pub enum ServerMessage {
    Stake(Vara),
    Unestake(Vara),
    Withdraw(Vara)
}