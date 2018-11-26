use serde::de::Deserialize;
use serde::de::DeserializeOwned;
use EosApi;
use serde_json::Value;

#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub mod basic;
pub mod abi;
pub mod chain;
pub mod dbsize;
pub mod history;
pub mod net;
pub mod producer;
pub mod error;

use self::basic::*;
use self::error::EosError;

pub mod all{
    pub use super::chain;
    pub use super::dbsize;
    pub use super::net;
    pub use super::history;
    pub use super::producer;
    pub use super::abi::ABI;
    pub use super::Transaction;
    pub use super::Pfunc;
    pub use super::EosResponse;
}
pub enum EosResponse<T>
where T: DeserializeOwned{
    Fine(T),
    Error(EosError)
}
impl<T> EosResponse<T>
where T: DeserializeOwned{
    pub fn parse(raw: Value) ->Self{
        match serde_json::from_value::<T>(raw.clone()){
             Ok(e) =>{
                 EosResponse::Fine(e)
             }
             Err(_)=>{
                let eoserror: EosError = serde_json::from_value(raw).unwrap();
                EosResponse::Error(eoserror)
             }
         }
    }
}

pub trait Pfunc<'a, T>
where
    T: Deserialize<'a>,
{
    fn response(&self, &EosApi<'a>) -> T;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Auth {
    actor: String,
    permission: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TActions {
    account: String,
    name: String,
    authorization: Vec<Auth>,
    data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub expiration: String,
    pub ref_block_num: u64,
    pub ref_block_prefix: u64,
    pub max_net_usage_words: u64,
    pub max_cpu_usage_ms: u64,
    pub delay_sec: u64,
    #[serde(default)]
    pub context_free_actions: Vec<TActions>,
    #[serde(default)]
    pub context_free_data: Vec<String>,
    #[serde(default)]
    pub actions: Vec<TActions>,
    #[serde(default)]
    pub signatures: Vec<String>,
    #[serde(default)]
    pub transaction_extensions: Vec<String>,
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction {
            expiration: "".to_string(),
            ref_block_num: 0,
            ref_block_prefix: 0,
            max_net_usage_words: 0,
            max_cpu_usage_ms: 0,
            delay_sec: 0,
            context_free_actions: vec![],
            context_free_data: vec![],
            actions: vec![],
            signatures: vec![],
            transaction_extensions: vec![],
        }
    }
}