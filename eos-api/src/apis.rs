use std::str;
use std::collections::HashMap;
use std::borrow::Cow;   

//api => key,method_name

#[derive(Clone)]
pub struct Apis<'a>{
    apis: HashMap<&'a str,(Cow<'a,str>,Cow<'a,str>)>
}
impl<'a> Apis<'a>{
    pub fn new()->Self{
        let mut apis= HashMap::new();
        let chain = vec!["get_info",
                          "get_block",
                          "get_block_header_state",
                          "get_account",
                          "get_abi",
                          "get_code",
                          "get_raw_code_and_abi",
                          "get_table_rows",
                          "get_currency_balance",
                          "abi_json_to_bin",
                          "abi_bin_to_json",
                          "get_required_keys",
                          "get_currency_state",
                          "get_producers",
                          "push_block",
                          "push_transaction"];
        let dbsize = vec!["get"];
        let history =vec!["get_actions",
                          "get_transaction",
                          "get_key_accounts",
                          "get_controlled_accounts"];
        let net   =  vec!["connect",
                          "disconnect",
                          "connections",
                          "status"];
        let producer=vec!["pause",
                          "resume",
                          "paused",
                          "get_runtime_options",
                          "update_runtime_options",
                          "get_greylist_accounts",
                          "get_whitelist_blacklist",
                          "set_whitelist_blacklist"];
        for method in chain.iter(){
            apis.insert(*method,("v1".into(),"chain".into()));
        }
        for method in net.iter(){
            apis.insert(*method,("v1".into(),"net".into()));
        }
        for method in dbsize.iter(){
            apis.insert(*method,("v1".into(),"dbsize".into()));
        }
        for method in history.iter(){
            apis.insert(*method,("v1".into(),"history".into()));
        }
        for method in producer.iter(){
            apis.insert(*method,("v1".into(),"producer".into()));
        }
        Apis{
            apis: apis
        }
    }
    pub fn index(&self, target: &str) -> String{
        let e=self.apis.get(target).unwrap();
        format!("{}/{}/{}",e.0,e.1,target)
    }
}