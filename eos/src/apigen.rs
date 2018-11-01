use eos_api::EosApi;
use std::collections::HashSet;
use abicache::AbiCache;
use serde_json::{self,Value};

struct ApiGen<'a>{
    network: &'a EosApi,
    abicache: &'a AbiCache
}
impl<'a> ApiGen<'a>{
    fn new(network: &'a EosApi,abicache: &'a AbiCache)->{
        ApiGen{
            network: network,
            abicache: abicache
        }
    }
    fn get_transaction<F>(&self,args: Vec<Value>,callback: F.option: String)
    where F: Fn(Value){
        let mut contract=args[0].clone();
        let mut accounts = HashSet::new();
        match  args[0]{
            Value::String(st) =>{
                contract=st;
            },
            Value::Array(arr) =>{
                
            },
            _ =>{
                
            }
        }
        for actions in args[0]["actions"].as_array().unwrap().iter(){
            accounts.insert(actions["account"].as_str().unwrap()_or(""));
        }
        
        let mut cachecode= HashSet::new();
        cachecode.insert("eosio");
        cachecode.insert("eosio.token");
        cachecode.insert("eosio.null");
        
        let mut abi_promise: Vec<ABI> = Vec::new();
        for acnt in accounts.iter(){
            if cachecode.contains(&acnt){
                let code_abi=self.abicache.abi_async(acnt);
                abi_promise.push(code_abi);
            }
        }
        if contract != json!(null){
            
        }
    }
}