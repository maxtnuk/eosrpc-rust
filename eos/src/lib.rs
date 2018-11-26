extern crate eos_api;
extern crate eos_type;
extern crate crypto;
extern crate regex;
extern crate num;
extern crate num_bigint;
extern crate bincode;

pub mod eosformat;
pub mod sign;

#[cfg(test)]
mod test;
pub mod prelude {
    pub use errorhandle::Errortype;
}

pub use eos_api::ApiConfig;
pub use eos_api::json_pretty;

use eos_api::EosApi;
pub use eos_api::form;
use form::all::*;

use eos_type::*;
use sign::SignProvider;

use std::collections::HashMap;

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

        let abis = HashMap::new();

        Eos {
            network: network,
            abis: abis,
            config: config,
        }
    }
    pub fn abi_async(&self, account: &str) -> ABI {
        let code = chain::request::GetRawCodeAndAbi
        { 
            account_name: account.to_string() 
        }.response(&self.network);
        code.abi.clone()
    }
    pub fn push_transaction(&self, block: Option<Transaction>) {
        let api = &self.network;
        let option = self.config.api_config.clone();
        let ref_block = block.unwrap_or({
            self.config.header.clone().unwrap_or({
                api.create_transaction(None)
            })
        });

        let reqkeys = chain::request::GetRequiredKeys {
            transaction: ref_block.clone(),
            available_keys: self.config.keys.clone(),
        }.response(api);

        let sigs = SignProvider {
            chain_id: option.chain_id.to_string(),
            keys: reqkeys,
            transaction: ref_block.clone(),
            abis: Vec::new(),
        }.gen_sigs();

        let sig_strings: Vec<String> = sigs.iter().map(|x| {
            match x {
                Ok(e) =>{
                    e.to_string()    
                },
                Err(_) =>{
                    "".to_string()
                }
            }
        }).collect();

        let packed_trx = chain::request::PushTransaction {
            compression: "none".to_string(),
            transaction: ref_block,
            context_free_data: "".to_string(),
            signatures: sig_strings,
        }.response(api);
    }
}
