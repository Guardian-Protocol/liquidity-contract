use gstd::{
    exec, 
    msg, 
    ActorId, ToString
};
use io::{
    ft_io::{
        FTAction, 
        FTError, 
        FTEvent
    }, 
    Gvara, LiquidError
};

use crate::secured_information;

pub async fn mint(amount: Gvara) -> Result<(), LiquidError> {
    let action: FTAction = FTAction::Mint { 
        amount: amount.clone(), 
        to: exec::program_id()
    };

    let result = msg::send_for_reply_as::<FTAction, Result<FTEvent, FTError>>(
        secured_information().gvara_token_address.clone(), 
        action, 
        0, 0
    ).expect("Error").await
        .expect("Internal contract error: code FT-01")
        .expect("Internal contract error: code FT-02");

    match result {
        FTEvent::Transferred { from: _, to: _, amount: _ }=> { Ok(()) },
        _ => {
            Err(LiquidError::InternalContractError("Internal contract error: please notify this to the dev team: code FT-03".to_string()))
        },
    }
}

pub async fn burn(amount: Gvara) -> Result<(), LiquidError> {
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

    match result {
        FTEvent::Transferred { from: _, to: _, amount: _ }=> { Ok(()) },
        _ => {
            Err(LiquidError::InternalContractError("Internal contract error: please notify this to the dev team: code FT-03".to_string()))
        },
    }
}

pub async fn transfer(amount: Gvara, from: ActorId, to: ActorId) -> Result<(), LiquidError> {
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

    match result {
        FTEvent::Transferred { from: _, to: _, amount: _ } => { Ok(()) },
        _ => {
            Err(LiquidError::InternalContractError("Internal contract error: please notify this to the dev team: code FT-03".to_string()))
        },
    }
}