use eos_api::EosApi;
use ecc::signature::Signature;
use ecc::key::{PrivateKey, PublicKey};
use structs::trs::Transaction;

use serde_json::Value;
#[derive(Clone)]
pub struct KeyProvider {
    pub public_key: Option<PublicKey>,
    pub private_key: Option<PrivateKey>,
}
impl Default for KeyProvider {
    fn default() -> Self {
        let randomkey = PrivateKey::unsafe_randomkey(None);
        KeyProvider {
            public_key: Some(PublicKey::from(&randomkey)),
            private_key: Some(randomkey),
        }
    }
}
impl From<String> for KeyProvider {
    fn from(data: String) -> Self {
        if PrivateKey::is_valid(data.clone()) {
            let result = PrivateKey::from(data.clone());
            KeyProvider {
                public_key: Some(PublicKey::from(&result)),
                private_key: Some(result),
            }
        } else if PublicKey::is_valid(data.clone(), None, None) {
            KeyProvider {
                public_key: Some(PublicKey::from(data.clone())),
                private_key: None,
            }
        } else {
            KeyProvider {
                public_key: None,
                private_key: None,
            }
        }
    }
}

pub struct SignProvider {
    keys: Vec<KeyProvider>,
}
impl SignProvider {
    pub fn new(keys: Vec<KeyProvider>) -> Self {
        SignProvider { keys: keys }
    }
    pub fn gen_sigs<T>(&self, network: &EosApi, tr: Transaction, buf: T) -> Vec<Signature>
    where
        T: Into<Vec<u8>>,
    {
        let buf = buf.into();
        if network.optional.http_endpoint.len() == 0 {
            return self.keys
                .iter()
                .filter(|ref x| x.private_key.clone().is_some())
                .fold(Vec::new(), |mut acc, x| {
                    let pvkey = x.private_key.clone().unwrap();
                    acc.push(Signature::sign(buf.as_slice(), &pvkey));
                    acc
                });
        }

        let pubkeys: Vec<String> = self.keys
            .iter()
            .filter(|x| x.public_key.clone().is_some())
            .map(|ref x| x.public_key.clone().unwrap().to_string())
            .collect();

        let tr_value = serde_json::to_value(tr).unwrap();
        let pubkeys_value: Vec<Value> = pubkeys.iter().map(|x| Value::String(x.clone())).collect();

        let req_value = {
            use serde_json::map::Map;
            let mut m = Map::new();
            m.insert(String::from("transaction"), tr_value);
            m.insert(String::from("available_keys"), Value::Array(pubkeys_value));
            Value::Object(m)
        };

        let respond = network
            .http_request("get_required_keys", &req_value)
            .unwrap();
        println!("{}", respond);
        let required_keys = respond["required_keys"].as_array().unwrap();

        let mut pvkeys = Vec::new();
        let mut missingkeys = Vec::new();
        for requirekey in required_keys.iter() {
            let reqpub = requirekey.as_str().unwrap();
            match self.keys.iter().find(|ref x| {
                x.public_key.clone().unwrap().to_string() == reqpub
            }) {
                Some(e) => {
                    pvkeys.push(e.private_key.clone().unwrap());
                }
                None => {
                    missingkeys.push(reqpub);
                }
            }
        }
        return pvkeys.iter().fold(Vec::new(), |mut acc, x| {
            acc.push(Signature::sign(buf.as_slice(), &x));
            acc
        });
    }
}
