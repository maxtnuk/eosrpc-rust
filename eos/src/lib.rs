extern crate eos_api;
extern crate ecc;
extern crate regex;
extern crate num;
extern crate num_bigint;
extern crate serde;
extern crate config;

#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod format;
pub mod errorhandle;
pub mod prelude;
pub mod abicache;
pub mod structs;
pub mod apigen;

#[cfg(test)]
mod test;

use eos_api::EosApi;
use abicache::AbiCache;
use abicache::serde_struct::ABI;
use config::Config;
use serde_json::{Value};


struct Eos{
    network: EosApi,
    abis: Vec<ABI>,
    abicache: AbiCache,
    structs: Value,
}
impl Eos{
    fn new(config: Option<Config>)->Self{
        
        let mut network=EosApi::new(config);
        if network.optional.get_str("chainId").is_err(){
            network.optional.set("chainId","cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f");
        }
        let mut config=network.optional.clone();
        
        let mut abis = Vec::new();
        let abicaches = AbiCache::new(&network,&config);
        
        let eosio_null = AbiCache::abi_value("eosio.null.abi.json");
        let eosio_token = AbiCache::abi_value("eosio.token.abi.json");
        let eosio_system = AbiCache::abi_value("eosio.system.abi.json");
        
        abis.push(AbiCache::abi("eosio-null",&eosio_null));
        abis.push(AbiCache::abi("eosio-token",&eosio_token));
        abis.push(AbiCache::abi("eosio-system",&eosio_system));
        
        Eos{
            network: network,
            abis: abis,
            abicache: abicaches,
            structs: json!(null)
        }
    }
    fn 
}
fn mergeWriteFunctions(config: Config,network: EosApi,abis: Vec<ABI>){
    let write_api = apigen::write_api_gen(&config,&network,&abis);
    
}