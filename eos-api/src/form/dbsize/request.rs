use Prov;
use EosCall;
use super::Pfunc;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetInfo;
impl<'a> Pfunc<'a, ReGetInfo> for GetInfo {
    fn response(&self, api: &EosApi<'a>) -> ReGetInfo {
        EosCall::new("get_info",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReGetInfo {
    pub server_version: String,
    pub head_block_num: u64,
    pub last_irreversible_block_num: u64,
    pub head_block_id: String,
    pub head_block_time: String,
    pub head_block_producer: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetRequired {
    pub transaction: Transaction,
    pub available_keys: Vec<String>,
}
impl<'a> Pfunc<'a, Requiredkeys> for GetRequired {
    fn response(&self, api: &EosApi<'a>) -> Requiredkeys {
        EosCall::new("get_required_keys",self.clone()).get_it(api)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Requiredkeys {
    pub require_keys: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiJsonToBin {
    pub code: String,
    pub action: String,
    pub args: Value,
}
impl<'a> Pfunc<'a, AbiBinCode> for AbiJsonToBin {
    fn get_it(&self, api: &EosApi<'a>) -> AbiBinCode {
        let result = api.http_request("abi_json_to_bin", &self).unwrap();
        serde_json::from_value(result).unwrap()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiProvider {
    pub account_name: String,
}
impl<'a> Pfunc<'a, RawAbiCode> for AbiProvider {
    fn get_it(&self, api: &EosApi<'a>) -> RawAbiCode {
        let result = api.http_request("get_raw_code_and_abi", &self).unwrap();
        serde_json::from_value(result).unwrap()
    }
}
