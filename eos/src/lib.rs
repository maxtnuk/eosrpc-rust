extern crate eos_api;
extern crate ecc;
extern crate regex;
extern crate num;
extern crate num_bigint;
extern crate serde;
extern crate chrono;
extern crate bincode;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod format;
pub mod errorhandle;
pub mod prelude;
pub mod structs;
pub mod eosecc;

#[cfg(test)]
mod test;

pub use eos_api::ApiConfig as ApiConfig;
pub use structs::trs::Transaction as Transaction;

use eos_api::EosApi;
use ecc::hash;

use structs::abi::{ABI};
use eosecc::{KeyProvider,SignProvider};

use serde_json::{Value};
use chrono::prelude::*;
use chrono::Duration;
use bincode::{serialize};
use std::collections::HashMap;
use std::fs::File;

pub struct EosConfig<'a>{
    pub api_config:ApiConfig<'a>,
    pub keys: Vec<KeyProvider>,
    pub header: Option<Transaction>,
}
impl<'a> Default for EosConfig<'a>{
   fn default() ->Self{
       EosConfig{
           api_config: Default::default(),
           keys: Vec::new(),
           header: None
       }
   }
}

pub struct Eos<'a>{
    network: EosApi<'a>,
    abis: HashMap<&'a str,ABI>,
    config: EosConfig<'a>
}
impl<'a> Eos<'a>{
    pub fn new(config: EosConfig<'a>)->Self{
        
        let network=EosApi::new(config.api_config.clone());
       
        let mut abis = HashMap::new();
        
        let eosio_null = Self::abi_value("eosio.null.abi.json");
        let eosio_token = Self::abi_value("eosio.token.abi.json");
        let eosio_system = Self::abi_value("eosio.system.abi.json");
        
        abis.insert("eosio-null",Self::abi(&eosio_null));
        abis.insert("eosio-token",Self::abi(&eosio_token));
        abis.insert("eosio-system",Self::abi(&eosio_system));
        
        Eos{
            network: network,
            abis: abis,
            config: config
        }
    }
    pub fn abi_async(&self,account: &str)->ABI{
        let account_info=json!({
            "account_name": account
        });
        let code=self.network.http_request("get_abi",&account_info).unwrap();
        Self::abi(&code["abi"])
    }
    pub fn abi(code: &Value)->ABI{
        serde_json::from_value(code.clone()).unwrap()
    }
    pub fn abi_value(file_name: &str)->Value{
        let default_path="./eos/src/schema/".to_string();
        let file = File::open(default_path+file_name).unwrap();
        serde_json::from_reader(file).unwrap_or(json!(null))
    }
    pub fn abi_to_bin(&self,code: String,action: String,args: &Value)->u64{
        use serde_json::map::Map;
        let req_value={
            let mut m=Map::new();
            m.insert(String::from("code"),Value::String(code));
            m.insert(String::from("action"),Value::String(action));
            m.insert(String::from("args"),args.clone());
            Value::Object(m)
        };
        let binresult=self.network.http_request("abi_json_to_bin",&req_value).unwrap();
        binresult["binargs"].as_u64().unwrap()
    }
    pub fn method(&self,name: &str, args: &str)->Value{
        let arg: Value=serde_json::from_str(args).unwrap();
        self.network.http_request(name,&arg).unwrap()
    }
    pub fn create_transaction(&self,expire_sec: Option<i64>)->Transaction{
        let api = &self.network;
        let get_info=api.http_request("get_info",&json!({})).unwrap();
        
        let head_block_time=get_info["head_block_time"].as_str().unwrap().to_owned();
        let chain_date = DateTime::parse_from_rfc3339((head_block_time+"Z").as_str()).unwrap();
        
        let irr_block=get_info["last_irreversible_block_num"].as_u64().unwrap();
        
        let block_param=json!({
            "block_num_or_id": irr_block
        });
        let block=api.http_request("get_block",&block_param).unwrap();
        //println!("{}",serde_json::to_string_pretty(&block).unwrap());
        let expiration = match expire_sec{
            Some(e) =>{
                chain_date+Duration::seconds(e*1000)
            },
            None =>{
                chain_date+Duration::seconds(60*1000)
            }
        };
        let ref_block_num = irr_block & 0xFFFF;
        
        Transaction{
            expiration: expiration.to_rfc3339(),
            ref_block_num: ref_block_num,
            ref_block_prefix: block["ref_block_prefix"].as_u64().unwrap(),
            ..Default::default()
        } 
    }
    pub fn transaction<F>(&self,block: Option<Transaction>,callback: F)
    where F: Fn(Value){
        let ref_block = block.unwrap_or({
            self.config.header.clone().unwrap_or({
                self.create_transaction(None)
            })
        });
        
        let buf: Vec<u8>=serialize(&ref_block).unwrap(); 
        let transaction_id=hash::to_hex(hash::sha256(buf.as_slice()));
        let option = self.config.api_config.clone();
        let sigs=if option.sign{
            let chainid_buf= option.chain_id.as_bytes().to_vec();
            let packed_context_freedata = vec![0u8;32];
            SignProvider::new(self.config.keys.clone())
                         .gen_sigs(&self.network,ref_block.clone(),buf.as_slice())
        }else{ Vec::new() };
        
        let sig_strings: Vec<Value>=sigs.iter().map(|x| Value::String(x.to_string()) ).collect();
        let ref_block_value=serde_json::to_value(ref_block).unwrap();
        
        let packed_trx={
            use serde_json::map::Map;
            let mut m=Map::new();
            m.insert(String::from("compression"),Value::String("none".to_string()));
            m.insert(String::from("packed_trx"),ref_block_value);
            m.insert(String::from("packed_context_free_data"),Value::String("".to_string()));
            m.insert(String::from("signatures"),Value::Array(sig_strings));
            Value::Object(m)
    };
        let api = &self.network;
        if option.broadcast{
            let result=api.http_request("push_transaction",&packed_trx);
            callback(result.unwrap());
        }else{
        
        }
    }
}