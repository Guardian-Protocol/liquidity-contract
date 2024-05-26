use gstd::{msg, String};
use io::{
    store_io::{
        StoreAction, 
        StoreError, 
        StoreResponse
    }, 
    UnestakeId, 
    Vara
};

use crate::secured_information;

pub async fn store_transaction(transaction_type: String, amount: Vara) -> Result<StoreResponse, StoreError> {
    let store_id = secured_information().users.entry(msg::source()).or_insert(find_store());

    let payload: StoreAction = StoreAction::StoreTransaction {
        user: msg::source(),
        transtaction_type: transaction_type,
        amount,
    };  

    return msg::send_for_reply_as::<StoreAction, Result<StoreResponse, StoreError>>(
        secured_information().store_contracts.get(*store_id).expect("Store not found").address.clone(), 
        payload, 
        0, 0
    ).expect("Internal contract error").await.expect("Internal contract error");
}

pub async fn store_unestake(amount: Vara, liberation_era: u64, liberation_days: u64) -> Result<StoreResponse, StoreError> {
    if let Some(store_id) = secured_information().users.get(&msg::source()) {
        let payload: StoreAction = StoreAction::StoreUnestake {
            user: msg::source().clone(),
            amount,
            liberation_era,
            liberation_days,
        };
    
        return msg::send_for_reply_as::<StoreAction, Result<StoreResponse, StoreError>>(
            secured_information().store_contracts.get(*store_id).expect("Store not found").address.clone(), 
            payload, 
            0, 0
        ).expect("Internal contract error").await.expect("Internal contract error");
    } else {
        return Err(StoreError::StoreNotAvailable)
    }
}

pub async fn delete_unestake(unestake_id: UnestakeId) -> Result<StoreResponse, StoreError> {
    if let Some(store_id) = secured_information().users.get(&msg::source()) {
        let payload: StoreAction = StoreAction::DeleteUnestake(unestake_id);
    
        return msg::send_for_reply_as::<StoreAction, Result<StoreResponse, StoreError>>(
            secured_information().store_contracts.get(*store_id as usize).expect("Store not found").address.clone(), 
            payload, 
            0, 0
        ).expect("Internal contract error").await.expect("Internal contract error");
    } else {
        return Err(StoreError::StoreNotAvailable)
    }
}

pub async fn fetch_unestake(unestake_id: UnestakeId) -> Result<StoreResponse, StoreError> {
    if let Some(store_id) = secured_information().users.get(&msg::source()) {
        let payload: StoreAction = StoreAction::FetchUnestake {
            user: msg::source().clone(),
            unestake_id,
        };
    
        return msg::send_for_reply_as::<StoreAction, Result<StoreResponse, StoreError>>(
            secured_information().store_contracts.get(*store_id as usize).expect("Store not found").address.clone(), 
            payload, 
            0, 0
        ).expect("Internal contract error").await.expect("Internal contract error");
    } else {
        return Err(StoreError::StoreNotAvailable)
    }
}

fn find_store() -> usize {
    if let Some(store_id) = secured_information().store_contracts.iter().position(|s| !s.is_full) {
        return store_id;
    } else {
        panic!("No store available")
    }
}