extern crate serde;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

use serde_json::Value;
use serde::ser::Serialize;
use std::fs::{self, File};
use std::collections::HashMap;
use std::path::PathBuf;
use serde::de::DeserializeOwned;
use std::str;
use std::borrow::Cow;
pub use serde_json::to_string_pretty as json_pretty;

use reqwest::Error;
pub mod form;
pub mod apis;

use form::all::*;
use apis::Apis;

#[cfg(test)]
mod test;

pub struct EosCall<'a,S>
where S: Serialize{
    method_name: Cow<'a,str>,
    input: S
}
impl<'a,S> EosCall<'a,S>
where S: Serialize{
    pub fn new<C>(method_name: C,input: S) ->Self
    where S: Serialize,
    C: Into<Cow<'a,str>>{
        EosCall{
            method_name: method_name.into(),
            input: input
        }
    }
    fn get_it<'b,T>(&self,api: &EosApi<'b>) -> T
     where T: DeserializeOwned {
         let result=api.http_request(&self.method_name, &self.input).unwrap();
         match EosResponse::parse(result){
             EosResponse::Fine(val) =>{
                val
             },
             EosResponse::Error(err) =>{
                panic!("{}",json_pretty(&err).unwrap());                 
             }
         }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApiConfig<'a> {
    pub http_endpoint: &'a str,
    pub port: &'a str,
    pub verbose: bool,
    pub debug: bool,
    pub broadcast: bool,
    pub sign: bool,
    pub chain_id: &'a str,
}
impl<'a> Default for ApiConfig<'a> {
    fn default() -> Self {
        ApiConfig {
            http_endpoint: "127.0.0.1",
            port:"8888",
            verbose: false,
            debug: false,
            broadcast: true,
            sign: true,
            chain_id: "cf057bbfb72640471fd910bcb67639c22df9f92470936cddc1ade0e2f2e7dc4f",
        }
    }
}

#[derive(Clone)]
pub struct EosApi<'a> {
    def: Apis<'a>,
    pub optional: ApiConfig<'a>,
}
impl<'a> EosApi<'a> {
    pub fn new(config: ApiConfig<'a>) -> Self {
        
        EosApi {
            def: Apis::new(),
            optional: config
        }
    }
    pub fn http_request<S>(&self, name: &str, body: &S) -> Result<Value, Error>
    where
        S: Serialize,
    {
        let httpurl = self.optional.http_endpoint;
        let port = self.optional.port;
        let url=format!("http://{}:{}/{}",httpurl,port,&self.def.index(name));

        println!("url: {}",url);
        let prepare=reqwest::Client::new().post(&url);
        
        let res = if serde_json::to_value(body).unwrap() == Value::Null{
            prepare.json(&json!({}))
        }else{
            prepare.json(body)
        }.send()?.json()?;
        
        Ok(res)
    }
    pub fn abi_value(path: &str) -> ABI {
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }
    pub fn abi_to_bin(&self, code: String, action: String, args: &Value) -> String {
        let binresult = chain::request::AbiJsonToBin {
            code: code,
            action: action,
            args: args.clone(),
        }.response(self);

        binresult.binargs
    }
}
