use {EosCall,EosApi};
use form::Pfunc;
use form::basic::*;
use super::response as re;
use serde_json::Value;
//all of theme not defined

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connect;
impl<'a> Pfunc<'a, Value> for Connect {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("connect",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Disconnect;
impl<'a> Pfunc<'a, Value> for Disconnect {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("disconnect",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connections;
impl<'a> Pfunc<'a, Value> for Connections {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("connections",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Status;
impl<'a> Pfunc<'a, Value> for Status {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("status",self.clone()).get_it(api)
    }
}
