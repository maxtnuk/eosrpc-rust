use form::basic::*;
use form::Auth;
use form::abi::ABI;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetInfo {
    pub server_version: String,
    pub chain_id: String,
    pub head_block_num: u64,
    pub last_irreversible_block_num: u64,
    pub last_irreversible_block_id: String,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
    pub virtual_block_cpu_limit: u64,
    pub virtual_block_net_limit: u64,
    pub block_cpu_limit: u64,
    pub block_net_limit: u64,
    pub server_version_string: String
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRequiredKeys {
    pub require_keys: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiJsonToBin {
    pub binargs: u64,
    pub required_scope: Vec<String>,
    pub required_auth: Vec<Auth>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRawCodeAndAbi {
    pub account_name: account_name,
    pub wasm: String, //bytes
    pub abi: ABI,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PushTransaction {
    pub transaction_id: String
}
#[derive(Serialize, Deserialize)]
pub struct Regions {
    pub region: i64,
    pub cycles_summary: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlock {
    pub action_mroot: String,
    #[serde(default)]
    pub block_extensions: Vec<Value>,
    pub block_num: u64,
    pub confirmed: u64,
    #[serde(default)]
    pub header_extensions: Vec<Value>,
    pub id: String,
    #[serde(default)]
    pub new_producers: Option<String>,
    pub previous: String,
    pub producer: String,
    pub producer_signature: String,
    pub ref_block_prefix: u64,
    pub schedule_version: u64,
    pub timestamp: String,
    #[serde(default)]
    pub transaction_mroot: String,
    #[serde(default)]
    pub transactions: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct GetAccount {
  account_name: String,
  permissions: Vec<Permissions>,
}

#[derive(Serialize, Deserialize)]
pub struct Keys {
  key: String,
  weight: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Permissions {
  perm_name: String,
  parent: String,
  required_auth: RequiredAuth,
}

#[derive(Serialize, Deserialize)]
pub struct RequiredAuth {
  threshold: i64,
  keys: Vec<Keys>,
  accounts: Vec<account>,
}
#[derive(Serialize, Deserialize)]
pub struct GetCode {
  name: String,
  code_hash: String,
  wast: String,
  abi: ABI
}
#[derive(Serialize, Deserialize)]
pub struct GetTableRows{
    pub rows: Vec<Value>,
    pub more: bool
}
#[derive(Serialize, Deserialize)]
pub struct AbiBinToJson{
    pub args: Value,
    pub required_scope: Vec<Value>,
    pub required_auth: Vec<RequiredAuth>
}