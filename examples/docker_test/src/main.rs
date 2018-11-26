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
    let result = chain::request::GetInfo {}.response(&test.network);
    println!("{}", json_pretty(&result).unwrap());
    //println!("{}",result);
    let trx = test.push_transaction(None);
    println!("{}", json_pretty(&trx).unwrap());
    /*
    test.transaction(Some(trx),|x|{
        println!("{}",x);
    });
    */
}
