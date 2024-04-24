use gstd::{format, msg, String};
use io::Gvara;

pub async fn stash_message(amount: Gvara, message_type: String) -> String {
    return format!("{{
        \"type\": \"{}\",
        \"amount\": {},
        \"source\": \"{:?}\",
        \"value\": {}
    }}",message_type, amount, msg::source().clone(), msg::value());
}