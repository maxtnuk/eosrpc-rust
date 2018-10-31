extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;
extern crate config;

use serde_json::{Value};
use std::fs::{self,File};
use std::collections::HashMap;
use std::path::PathBuf;
use config::{Config};

use chrono::prelude::*;
use chrono::Duration;
use reqwest::Error;

#[cfg(test)]
mod test;

type Method=HashMap<String,Argu>;

#[derive(Serialize, Deserialize,Debug)]
struct Argu{
    #[serde(default)]
    brief: String,
    params: Value,
    results: Value,
}
pub struct EosApi{
    def: HashMap<String,Method>,
    optional: Config
}
impl EosApi{
    pub fn new(in_config: Option<Config>)->Self{
        let mut apis :HashMap<String,Method>= HashMap::new();
        
        for entry in fs::read_dir("src/apis").unwrap(){
            let entry =entry.unwrap();
            let version_str=entry.path().as_path().iter()
                            .skip(2).collect::<PathBuf>()
                            .to_string_lossy().into_owned();
            for api_file in fs::read_dir(entry.path()).unwrap(){
                let api_file =api_file.unwrap();
                let file_name = api_file.path().file_stem().unwrap()
                                        .to_string_lossy()
                                        .into_owned();
                let api_path= version_str.clone() + "/" + &file_name;
                
                //println!("{}",api_path);
                
                let file = File::open(api_file.path()).unwrap();
                let u :Method= serde_json::from_reader(file).unwrap();
                apis.insert(api_path,u);
            }
        }
        let mut default_config= Config::default();
        default_config.set("httpEndpoint","http://127.0.0.1:8888").unwrap();
        default_config.set("verbose",false).unwrap();
        
        EosApi{
            def: apis,
            optional: in_config.unwrap_or(default_config)
        }
    }
    fn http_request(&self,name: String,body: &serde_json::Value) -> Result<Value, Error> {
        let httpurl=self.optional.get::<String>("httpEndpoint").unwrap();
        let mut url=String::new();
        for (k,v) in self.def.iter(){
            if v.get(&name).is_some() {
                url=format!("{}/{}/{}",httpurl,k,name);
                break; 
            }
        };
        println!("url: {}",url);
        let res = reqwest::Client::new()
            .post(&url)
            .json(body)
            .send()?
            .json()?;
        Ok(res)
    }
}
pub fn create_transaction<F>(api: &EosApi,expire_sec: Option<i64>,callback: F)
where F: Fn(Value){
    let get_info=api.http_request("get_info".to_string(),&Value::Null).unwrap();
                 
    let head_block_time=get_info["head_block_time"].as_str().unwrap().to_owned();
    let chain_date = DateTime::parse_from_rfc3339((head_block_time+"Z").as_str()).unwrap();
    
    let irr_block=get_info["last_irreversible_block_num"].as_u64().unwrap();
    
    let block_param=json!({
        "block_num_or_id": irr_block
    });
    let block=api.http_request("get_block".to_string(),&block_param).unwrap();
    
    let expiration = match expire_sec{
        Some(e) =>{
            chain_date+Duration::seconds(e*1000)
        },
        None =>{
            chain_date+Duration::seconds(60*1000)
        }
    };
    let ref_block_num = irr_block & 0xFFFF;
    
    let block_info = json!({
        "expiration": expiration.to_rfc3339(),
        "ref_block_num": ref_block_num,
        "ref_block_prefix": block["ref_block_prefix"].as_str().unwrap(),
        "max_net_usage_words": 0,
        "max_cpu_usage_ms": 0,
        "delay_sec": 0,
        "context_free_actions": [],
        "actions": [],
        "signatures": [],
        "transaction_extensions": []
    });
                     
    callback(block_info);  
} 
