extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;

use chrono::prelude::*;
use chrono::Duration;
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

#[macro_export]
macro_rules! eos_call {
    ($e:expr, $($json:tt)+) => {{
        let input=json!($($json)+);
        let method=$e.to_string();
        EosCall::new(method,input)
    }}
}
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
         serde_json::from_value(result).unwrap()
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
            optional: config,
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
        let res = reqwest::Client::new().post(&url).json(body).send()?.json()?;
        Ok(res)
    }
    pub fn create_transaction(&self, expire_sec: Option<i64>) -> Transaction {
        let api = self;
        let get_info = chain::request::GetInfo {}.response(api);

        let head_block_time = get_info.head_block_time;
        let chain_date = DateTime::parse_from_rfc3339((head_block_time + "Z").as_str()).unwrap();

        let irr_block = get_info.last_irreversible_block_num;

        let block = chain::request::GetBlock { block_num_or_id: irr_block }.response(api);

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
}
