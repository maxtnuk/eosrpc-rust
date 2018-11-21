use eos_api::EosApi;
use crypto::signature::{Signature,ResultSignature};
use crypto::key::PrivateKey;
use rpc_interface::*;
use bincode::serialize;
use serde::de::Deserialize;

use serde_json::{self, Value};

pub mod eosstruct;

pub trait Pfunc<'a, T>
where
    T: Deserialize<'a>,
{
    fn get_it(&self, &EosApi<'a>) -> T;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthorityProvider {
    pub transaction: Transaction,
    pub available_keys: Vec<String>,
}
impl<'a> Pfunc<'a, Requiredkeys> for AuthorityProvider {
    fn get_it(&self, api: &EosApi<'a>) -> Requiredkeys {
        let result = api.http_request("get_required_keys", &self).unwrap();
        serde_json::from_value(result).unwrap()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AbiToBinProvider {
    pub code: String,
    pub action: String,
    pub args: Value,
}
impl<'a> Pfunc<'a, AbiBinCode> for AbiToBinProvider {
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
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PushTransactionProvider {
    pub compression: String,
    pub transaction: Transaction,
    pub context_free_data: String,
    pub signatures: Vec<String>,
}
//not defined
impl<'a> Pfunc<'a, Value> for PushTransactionProvider {
    fn get_it(&self, api: &EosApi<'a>) -> Value {
        let result = api.http_request("push_transaction", &self).unwrap();
        result
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InfoProvider;
impl<'a> Pfunc<'a, Info> for InfoProvider {
    fn get_it(&self, api: &EosApi<'a>) -> Info {
        let result = api.http_request("get_info", &json!({})).unwrap();
        serde_json::from_value(result).unwrap()
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlcokProvoder {
    pub block_num_or_id: u64,
}
impl<'a> Pfunc<'a, BlockInfo> for BlcokProvoder {
    fn get_it(&self, api: &EosApi<'a>) -> BlockInfo {
        let result = api.http_request("get_block", &self).unwrap();
        //println!("{}", result);
        serde_json::from_value(result).unwrap()
    }
}
pub struct SignProvider {
    pub chain_id: String,
    pub keys: Requiredkeys,
    pub transaction: Transaction,
    pub abis: Vec<ABI>,
}
impl SignProvider {
    pub fn gen_sigs(&self) -> Vec<ResultSignature> {
        use crypto::to_bytes;
        let mut buf = to_bytes(self.chain_id.to_string());
        let serialtrx = serialize(&self.transaction).unwrap();
        buf.extend(serialtrx);
        buf.extend(vec![0u8; 32]);
        self.keys.require_keys.iter().fold(
            Vec::new(),
            |mut acc, x| {
                let pvkey = PrivateKey::from(x.as_str());
                acc.push(Signature::sign(buf.as_slice(), &pvkey));
                acc
            },
        )
    }
}
