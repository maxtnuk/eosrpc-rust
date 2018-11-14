#![feature(proc_macro_hygiene)]
//extern crate eosio;
//extern crate eosrpc_rust;

use eosio::*;

use eos::{Eos, EosConfig, ApiConfig};
use eos::{InfoProvider, Pfunc, json_pretty};

pub mod abi;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_config = EosConfig {
            api_config: ApiConfig {
                http_endpoint: "http://172.18.0.2:8888",
                ..Default::default()
            },
            ..Default::default()
        };
        let test = Eos::new(test_config);
        let result = InfoProvider {}.get_it(&test.network);
        println!("{}", json_pretty(&result).unwrap());
        //println!("{}",result);
        let trx = test.create_transaction(None);
        println!("{}", json_pretty(&trx).unwrap());
    }
}
