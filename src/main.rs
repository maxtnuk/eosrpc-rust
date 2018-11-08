extern crate eos;

use eos::{Eos,EosConfig,ApiConfig};
fn main(){
    let test_config=EosConfig{
        api_config: ApiConfig{
            http_endpoint: "http://172.18.0.2:8888",
            ..Default::default()   
        },
        ..Default::default()
    };
    let test=Eos::new(test_config);
    let input=r#"{ 
                "account_name" : "eosio" 
        }"#;
    let result=test.method("get_info","{}");
    //println!("{}",result);
    let trx=test.create_transaction(None);
    println!("{:?}",trx);
    /*
    test.transaction(Some(trx),|x|{
        println!("{}",x);
    });
    */
}