use Prov;
use EosCall;
use form::Pfunc;
use form::basic::*;
use super::request as re;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetInfo;
impl<'a> Pfunc<'a, re::GetInfo> for GetInfo {
    fn response(&self, api: &EosApi<'a>) -> re::GetInfo {
        EosCall::new("get_info",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetBlock {
    pub block_num_or_id: u64,
}
impl<'a> Pfunc<'a, re::GetBlock> for GetBlock {
    fn response(&self, api: &EosApi<'a>) -> re::GetBlock {
         EosCall::new("get_block",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAccount {
    pub account_name: account_name,
}
impl<'a> Pfunc<'a, re::GetAccount> for GetAccount {
    fn response(&self, api: &EosApi<'a>) -> re::GetAccount {
         EosCall::new("get_account",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAbi {
    pub account_name: account_name,
}
//not define
impl<'a> Pfunc<'a, Value> for GetAbi {
    fn response(&self, api: &EosApi<'a>) -> Value {
         EosCall::new("get_abi",self.clone()).get_it(api)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCode {
    pub account_name: account_name,
}
impl<'a> Pfunc<'a, re::GetCode> for GetCode {
    fn response(&self, api: &EosApi<'a>) -> re::GetCode {
         EosCall::new("get_code",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRawCodeAndAbi {
    pub account_name: String,
}
impl<'a> Pfunc<'a, re::GetRawCodeAndAbi> for GetRawCodeAndAbi {
   fn response(&self, api: &EosApi<'a>) -> re::GetRawCodeAndAbi {
        EosCall::new("get_raw_code_and_abi",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug,Default)]
pub struct GetTableRows {
    pub scope: String,
    pub code: String,
    pub table: String,
    pub json: bool,
    pub lower_bound: i32,
    pub upper_bound: i32,
    pub limit: i32
}
impl<'a> Pfunc<'a, re::GetTableRows> for GetTableRows {
    fn response(&self, api: &EosApi<'a>) -> re::GetTableRows {
        EosCall::new("get_table_rows",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCurrencyBalance {
    pub code: String,
    pub account: account,
    pub symbol: String
}
//not defined
impl<'a> Pfunc<'a, Value> for GetCurrencyBalance {
    fn response(&self, api: &EosApi<'a>) -> Value {
        EosCall::new("get_currency_balance",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiJsonToBin {
    pub code: String,
    pub action: String,
    pub args: Value,
}
impl<'a> Pfunc<'a, re::AbiJsonToBin> for AbiJsonToBin {
    fn response(&self, api: &EosApi<'a>) -> re::AbiJsonToBin {
        EosCall::new("abi_json_to_bin",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiBinToJson {
    pub code: String,
    pub action: String,
    pub binargs: String,
}
impl<'a> Pfunc<'a, re::AbiBinToJson> for AbiBinToJson {
    fn response(&self, api: &EosApi<'a>) -> re::AbiBinToJson {
        EosCall::new("abi_bin_to_json",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRequiredKeys {
    pub transaction: Transaction,
    pub available_keys: Vec<String>,
}
impl<'a> Pfunc<'a, re::GetRequiredKeys> for GetRequiredKeys {
    fn response(&self, api: &EosApi<'a>) -> re::GetRequiredKeys {
        EosCall::new("get_required_keys",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PushTransaction {
    pub compression: String,
    pub transaction: Transaction,
    pub context_free_data: String,
    pub signatures: Vec<String>,
}
impl<'a> Pfunc<'a, re::PushTransaction> for PushTransaction {
    fn response(&self, api: &EosApi<'a>) -> re::PushTransaction {
        EosCall::new("push_transaction",self.clone()).get_it(api)
    }
}
