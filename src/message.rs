use bincode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Ping,
    Pong,
    Transaction {
        from: String,
        to: String,
        amount: u64,
    },
}
