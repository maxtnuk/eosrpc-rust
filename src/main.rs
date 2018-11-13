extern crate eos;
use eos::{Eos, EosConfig, ApiConfig};
use eos::{InfoProvider, Pfunc};
fn main() {
    unimplemented!()
}
#[test]
fn eos_test() {
    let test_config = EosConfig {
        api_config: ApiConfig {
            http_endpoint: "http://172.18.0.2:8888",
            ..Default::default()
        },
        ..Default::default()
    };
    let test = Eos::new(test_config);
    let result = InfoProvider {}.get_it(&test.network);
    println!("{:?}", result);
    //println!("{}",result);
    let trx = test.create_transaction(None);
    println!("{:?}", trx);
    /*
    test.transaction(Some(trx),|x|{
        println!("{}",x);
    });
    */
}
