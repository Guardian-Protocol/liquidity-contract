use gstd::{
    format, 
    msg, 
    String
};
use io::server_io::ServerMessage;

use crate::secured_information;

pub fn server_message(message: ServerMessage) {

    let value: u128 = msg::value();
    
    let payload: String = match message {
        ServerMessage::Stake(amount) => {
            format!("{{
                \"type\": \"stake\",
                \"amount\": {},
                \"source\": \"{:?}\",
                \"value\": {}
            }}", amount, msg::source().clone(), value)
        },
        ServerMessage::Unestake(amount) => {
            format!("{{
                \"type\": \"unestake\",
                \"amount\": {},
                \"source\": \"{:?}\",
                \"value\": {}
            }}", amount, msg::source().clone(), value)
        },
        ServerMessage::Withdraw(amount) => {
            format!("{{
                \"type\": \"withdraw\",
                \"amount\": {},
                \"source\": \"{:?}\",
                \"value\": {}
            }}", amount, msg::source().clone(), value)
        }
    };

   let _ = msg::send(secured_information().stash_account_address, payload, value);

}