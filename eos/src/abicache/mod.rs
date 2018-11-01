use eos_api::EosApi;
use config::Config;
use serde_json::{self,Value};
use std::fs::File;
pub mod serde_struct;

use self::serde_struct::ABI;
pub struct AbiCache{
    network: EosApi,
    config: Config
}
impl AbiCache{
    pub fn new(network: &EosApi,config: &Config) -> Self{
        AbiCache{
            network: network.clone(),
            config: config.clone()
        }
    }
    pub fn abi_async(&self,account: &str)->ABI{
        let account_info=json!({
            "account_name": account
        });
        let code=self.network.http_request("get_abi",&account_info).unwrap();
        Self::abi(account,&code["abi"])
    }
    pub fn abi(account: &str,code: &Value)->ABI{
        serde_json::from_value(code.clone()).unwrap()
    }
    pub fn abi_value(file_name: &str)->Value{
        let default_path="src/schema/".to_string();
        let file = File::open(default_path+file_name).unwrap();
        serde_json::from_reader(file).unwrap_or(json!(null))
    }
}
