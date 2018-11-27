extern crate serde;
extern crate serde_yaml;
extern crate eos;
use eos::{Eos, EosConfig, ApiConfig};
use eos::{json_pretty};
use eos::form::all::*;
use serde_yaml::Value;
use std::fs::File;

fn main() {
    let setting: Value=serde_yaml::from_reader(File::open("src/setting.yaml").unwrap()).unwrap();
    let test_config = EosConfig {
        api_config: ApiConfig {
            http_endpoint: setting["server_ip"].as_str().unwrap(),
            //http_endpoint: "http://172.172.18.0.2:8888",
            ..Default::default()
        },
        ..Default::default()
    };
    let test = Eos::new(test_config);
    let trx = chain::request::GetInfo {}.response(&test.network);
    println!("{}", json_pretty(&trx).unwrap());
    //println!("{}",result);
    let mut trx=test.create_transaction(None);
    /*
    let auth =Auth{
        actor: "eosio".to_string(),
        permission: "active".to_string()
    };
    test.config_transaction(&mut trx,&req,"eosio".to_string(),vec![auth]);
    let result = test.push_transaction(Some(trx));
    println!("{}", result);
    test.transaction(Some(trx),|x|{
        println!("{}",x);
    });
    */
}
