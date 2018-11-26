use {EosCall,EosApi};
use form::Pfunc;
use form::basic::*;
use super::response as re;
use serde_json::Value;
//all of them not defined
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetActions {
    pub pos: i32,
    pub offset: i32,
    pub account_name: String,
}
impl<'a> Pfunc<'a, Value> for GetActions {
    fn response(&self, api: &EosApi<'a>) -> Value {
         EosCall::new("get_actions",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetTransaction {
    pub id: String
}
impl<'a> Pfunc<'a, Value> for GetTransaction {
    fn response(&self, api: &EosApi<'a>) -> Value {
         EosCall::new("get_transaction",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetKeyAccounts {
    pub public_key: String,
}
impl<'a> Pfunc<'a, Value> for GetKeyAccounts {
    fn response(&self, api: &EosApi<'a>) -> Value {
         EosCall::new("get_key_accounts",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetControlledAccounts {
    pub controlling_account: String,
}
impl<'a> Pfunc<'a, Value> for GetControlledAccounts {
    fn response(&self, api: &EosApi<'a>) -> Value {
         EosCall::new("get_controlled_accounts",self.clone()).get_it(api)
    }
}