pub mod response;
pub mod trs;

pub use response::*;
pub use trs::Transaction;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    actor: String,
    permission: String,
}
