use {EosCall,EosApi};
use form::Pfunc;
use form::basic::*;
use super::response as re;
use serde_json::Value;
//all of theme not defined

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Get;
impl<'a> Pfunc<'a, Value> for Get {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("get",self.clone()).get_it(api)
    }
}