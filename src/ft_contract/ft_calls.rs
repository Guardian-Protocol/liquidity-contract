use gstd::{errors::Error, exec, msg, ActorId};
use io::{
    ft_io::{FTAction, FTError, FTEvent}, 
    Gvara
};

use crate::secured_information;

pub async fn mint(amount: Gvara) {
    let action: FTAction = FTAction::Mint { 
        amount: amount.clone(), 
        to: exec::program_id()
    };

    let result = msg::send_for_reply_as::<FTAction, Result<FTEvent, FTError>>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await.expect("Internal FT contract error");

    let _ = match result {
        Ok(FTEvent::Transferred { from, to, amount }) => { 
            // Ok
        },
        Err(_) => {
            panic!("The caller is not an admin")
        },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        }
    };
}

pub async fn burn(amount: Gvara) {
    let action: FTAction = FTAction::Burn {
        amount: amount.clone() 
    };

    let result: FTEvent = msg::send_for_reply_as::<FTAction, Result<FTEvent, FTError>>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await
        .expect("Internal contract error: code FT-02")
        .expect("Internal contract error: code FT-03");

    let _ = match result {
        FTEvent::Transferred { from, to, amount } => { },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        },
    };
}

pub async fn transfer(amount: Gvara, from: ActorId, to: ActorId) {
    let action: FTAction = FTAction::Transfer { 
        tx_id: None, 
        from: from.clone(), 
        to: to.clone(), 
        amount: amount.clone() 
    };

    let result: FTEvent = msg::send_for_reply_as::<FTAction, Result<FTEvent, FTEvent>>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await
        .expect("Internal contract error: code FT-02")
        .expect("Internal contract error: code FT-03");

    let _ = match result {
        FTEvent::Transferred { from, to, amount } => { },
        _ => {
            panic!("Internal contract error: please notify this to the dev team: code FT-01")
        },
    };
}