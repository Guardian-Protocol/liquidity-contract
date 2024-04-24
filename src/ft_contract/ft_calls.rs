use gstd::{msg, ActorId};
use io::{
    ft_io::{FTAction, FTEvent}, 
    Gvara
};

use crate::secured_information;

pub async fn mint(amount: Gvara) {
    let action: FTAction = FTAction::Mint(amount);

    let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await.expect("Internal contract error: code FT-02");

    let _ = match result {
        FTEvent::Ok => { },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        },
    };
}

pub async fn burn(amount: Gvara) {
    let action: FTAction = FTAction::Burn(amount);

    let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await.expect("Internal contract error: code FT-02");

    let _ = match result {
        FTEvent::Ok => { },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        },
    };
}

pub async fn transfer(amount: Gvara, from: ActorId, to: ActorId) {
    let action: FTAction = FTAction::Transfer {
        from: from.clone(),
        to: to.clone(),
        amount: amount.clone(),
    };

    let result: FTEvent = msg::send_for_reply_as::<FTAction, FTEvent>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await.expect("Internal contract error: code FT-02");

    let _ = match result {
        FTEvent::Ok => { },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        },
    };
}