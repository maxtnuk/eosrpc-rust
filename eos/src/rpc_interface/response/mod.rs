pub mod abi;
pub mod resptrs;
pub mod block;

pub use abi::ABI;
pub use resptrs::ResTransaction;
pub use block::BlockInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Requiredkeys {
    pub require_keys: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RawAbiCode {
    pub account_name: String,
    pub wasm: String, //bytes
    pub abi: ABI,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiBinCode {
    pub binargs: u64,
    pub required_scope: Vec<String>,
    pub required_auth: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Info {
    pub server_version: String,
    pub head_block_num: u64,
    pub last_irreversible_block_num: u64,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
}
