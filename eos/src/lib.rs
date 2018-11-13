extern crate eos_api;
extern crate eos_type;
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
pub mod prelude;
pub mod provider;
pub mod rpc_interface;

#[cfg(test)]
mod test;

pub use eos_api::ApiConfig;
pub use rpc_interface::trs::Transaction;

use eos_api::EosApi;

pub use rpc_interface::*;
use eos_type::*;
pub use provider::*;
pub use provider::Pfunc;

use serde_json::Value;
use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::fs::File;

pub struct EosConfig<'a> {
    pub api_config: ApiConfig<'a>,
    pub keys: Vec<String>,
    pub header: Option<Transaction>,
}
impl<'a> Default for EosConfig<'a> {
    fn default() -> Self {
        EosConfig {
            api_config: Default::default(),
            keys: Vec::new(),
            header: None,
        }
    }
}

pub struct Eos<'a> {
    pub network: EosApi<'a>,
    abis: HashMap<&'a str, ABI>,
    config: EosConfig<'a>,
}
impl<'a> Eos<'a> {
    pub fn new(config: EosConfig<'a>) -> Self {

        let network = EosApi::new(config.api_config.clone());

        let mut abis = HashMap::new();

        let eosio_null = Self::abi_value("eosio.null.abi.json");
        let eosio_token = Self::abi_value("eosio.token.abi.json");
        let eosio_system = Self::abi_value("eosio.system.abi.json");

        abis.insert("eosio-null", eosio_null);
        abis.insert("eosio-token", eosio_token);
        abis.insert("eosio-system", eosio_system);

        Eos {
            network: network,
            abis: abis,
            config: config,
        }
    }
    pub fn abi_async(&self, account: &str) -> ABI {
        let code = AbiProvider { account_name: account.to_string() }.get_it(&self.network);
        code.abi.clone()
    }
    pub fn abi_value(file_name: &str) -> ABI {
        let default_path = "./eos/src/schema/".to_string();
        let file = File::open(default_path + file_name).unwrap();
        serde_json::from_reader(file).unwrap()
    }
    pub fn abi_to_bin(&self, code: String, action: String, args: &Value) -> u64 {
        let binresult = AbiToBinProvider {
            code: code,
            action: action,
            args: args.clone(),
        }.get_it(&self.network);

        binresult.binargs
    }
    pub fn method(&self, name: &str, args: &str) -> Value {
        let arg: Value = serde_json::from_str(args).unwrap();
        self.network.http_request(name, &arg).unwrap()
    }
    pub fn create_transaction(&self, expire_sec: Option<i64>) -> Transaction {
        let api = &self.network;
        let get_info = InfoProvider {}.get_it(api);

        let head_block_time = get_info.head_block_time;
        let chain_date = DateTime::parse_from_rfc3339((head_block_time + "Z").as_str()).unwrap();

        let irr_block = get_info.last_irreversible_block_num;

        let block = BlcokProvoder { block_num_or_id: irr_block }.get_it(api);

        let expiration = match expire_sec {
            Some(e) => chain_date + Duration::seconds(e * 1000),
            None => chain_date + Duration::seconds(60 * 1000),
        };
        let ref_block_num = irr_block & 0xFFFF;

        Transaction {
            expiration: expiration.to_rfc3339(),
            ref_block_num: ref_block_num,
            ref_block_prefix: block.ref_block_prefix,
            ..Default::default()
        }
    }
    pub fn push_transaction(&self, block: Option<Transaction>) {
        let api = &self.network;
        let option = self.config.api_config.clone();
        let ref_block = block.unwrap_or({
            self.config.header.clone().unwrap_or({
                self.create_transaction(None)
            })
        });

        let reqkeys = AuthorityProvider {
            transaction: ref_block.clone(),
            available_keys: self.config.keys.clone(),
        }.get_it(api);

        let sigs = SignProvider {
            chain_id: option.chain_id.to_string(),
            keys: reqkeys,
            transaction: ref_block.clone(),
            abis: Vec::new(),
        }.gen_sigs();

        let sig_strings: Vec<String> = sigs.iter().map(|x| x.to_string()).collect();

        let packed_trx = PushTransactionProvider {
            compression: "none".to_string(),
            transaction: ref_block,
            context_free_data: "".to_string(),
            signatures: sig_strings,
        }.get_it(api);
    }
}
