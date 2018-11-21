const PREFIX: &str = "EOS";

pub mod keyutil;

use regex::Regex;
use std::any::Any;
use secp256k1::{PublicKey as Pubkey,SecretKey as PvKey};
use secp256k1::SharedSecret;
use secp256k1::Error;
use prelude::*;

type ResultParse = Result<(PrivateKey, String, String), Errortype>;
type ResultPoint = Result<Pubkey, Errortype>;

#[derive(Clone)]
pub struct PublicKey {
    pub pubkey: Pubkey
}
impl PublicKey {
    pub fn new<'a>(
        q: &Any,
        prefix: Option<&'a str>,
    ) -> Result<Self, Errortype> {

        if let Some(string) = q.downcast_ref::<String>() {
            let in_str = Self::str_prefix(prefix);
            return match Self::from_string(string.clone(), in_str) {
                Ok(point) => {
                    Ok(PublicKey{
                        pubkey:point
                    })
                }
                Err(err) => {
                    err_print(err);
                    Err(Errortype::MakeFail {
                        who: "PublicKey".to_string(),
                        content: Some(string.clone()),
                    })
                }
            };
        }
        if let Some(vecs) = q.downcast_ref::<Vec<u8>>() {
            return match Self::from_data(vecs.as_slice()) {
                Ok(point) => {
                   Ok(PublicKey{
                        pubkey:point
                    })
                }
                Err(_) => {
                    Err(Errortype::MakeFail {
                        who: "PublicKey".to_string(),
                        content: None,
                    })
                }
            };
        }
        Err(Errortype::InputWrong)
    }
    pub fn is_valid<'a>(pubkey: String, prefix: Option<&'a str>) -> bool {
        Self::new(&pubkey, prefix).is_ok()
    }
    fn from_string(public_key: String, prefix: &str) -> ResultPoint {
        let re = Regex::new(r"^PUB_([A-Za-z0-9]+)_([A-Za-z0-9]+)$").unwrap();

        let decode_result = if !re.is_match(public_key.as_str()) {
            let prefix_match = Regex::new(("^".to_owned() + prefix).as_str()).unwrap();
            let result = if prefix_match.is_match(public_key.as_str()) {
                public_key.clone().split_off(prefix.len())
            } else {
                public_key.clone()
            };
            keyutil::check_decode(Data::from(result), None)
        } else {

            let key_info = re.captures(public_key.as_str()).unwrap();
            //println!("{}",key_info.len());

            let keytype = key_info.get(1).map_or("", |m| m.as_str());
            let keystring = key_info.get(2).map_or("", |m| m.as_str());

            //println!("{} {}",keytype,keystring);
            keyutil::check_decode(Data::from(keystring), Some(keytype.to_string()))
        };
        match decode_result {
            Ok(val) => {
                Ok(Self::from(val).pubkey)
            },
            Err(err) => {
                err_print(err);
                Err(Errortype::MakeFail {
                    who: "PublicKey".to_string(),
                    content: None,
                })
            }
        }
    }
    fn from_data<'a, S>(data: S) -> Result<Pubkey, Error>
    where
        S: Into<Data<'a>>,
    {
        let data=data.into();
        let compressed=data.len() != 65;
        Pubkey::parse_slice(data.as_ref(),Some(compressed))
    }
    pub fn to_string_with_type<'a>(&self, str_type: Option<&'a str>) -> String {
        Self::str_prefix(str_type).to_owned() + &keyutil::check_encode(self.to_vec(), None)
    }
    pub fn to_vec<'a>(&self) -> Data<'a> {
        Data::from(self.pubkey.serialize_compressed().to_vec())
    }
    fn str_prefix<'a>(prefix: Option<&'a str>) -> &'a str {
        match prefix {
            Some(e) => e,
            None => PREFIX,
        }
    }
}

#[derive(Clone)]
pub struct PrivateKey {
    pub pvkey: PvKey,
}
impl PrivateKey {
    pub fn new<T>(q: T) -> Self
    where
        T: Into<Self>,
    {
       q.into()
    }
    //{privatekey,format,keytype}
    fn parse(private_key: String) -> ResultParse {
        let re = Regex::new(r"^PVT_([A-Za-z0-9]+)_([A-Za-z0-9]+)$").expect("regex not create");

        if !re.is_match(private_key.as_str()) {
            return match keyutil::check_decode(
                Data::from(private_key),
                Some("sha256x2".to_string()),
            ) {
                Ok(versionkey) => {
                    let version = versionkey.clone();
                    if version[0] == 0x80 {
                        //print
                    }
                    let privateKey = Self::from(&versionkey[1..]);
                    let (keytype, format) = ("K1".to_string(), "WIF".to_string());
                    Ok((privateKey, format, keytype))
                }
                Err(err) => {
                    err_print(err);
                    Err(Errortype::ParseFail)
                }
            };
        }
        match re.captures(private_key.as_str()) {
            Some(key_info) => {
                let keytype = key_info.get(1).map_or("", |m| m.as_str());
                let keystring = key_info.get(2).map_or("", |m| m.as_str());

                match keyutil::check_decode(Data::from(keystring), Some(keytype.to_string())) {
                    Ok(pri_data) => {
                        let privateKey = Self::from(pri_data);
                        Ok((
                            privateKey,
                            "PVT".to_string(),
                            key_info["keytype"].to_string(),
                        ))
                    }
                    Err(err) => {
                        err_print(err);
                        Err(Errortype::ParseFail)
                    }
                }
            }
            None => Err(Errortype::ParseFail),
        }
    }
    fn to_wif(&self) -> String {
        let mut private_key = self.to_vec();
        private_key.insert(0, 0x80);
        keyutil::check_encode(Data::new(private_key), Some("sha256x2".to_string()))
    }
    fn to_vec<'a>(&self) -> Vec<u8> {
        let b_data = self.pvkey.serialize().to_vec();
        if b_data.len() < 32 {
            let zeros = vec![0u8; 32 - b_data.len()];
            zeros.iter().chain(b_data.iter()).map(|x| *x).collect()
        } else {
            b_data
        }
    }
    pub fn get_shared_secret<'a>(&self, pubkey: &PublicKey) -> Data<'a> {
        let shkey=SharedSecret::new(&pubkey.pubkey,&self.pvkey).unwrap();
        Data::new(shkey.as_ref().to_vec())
    }
    fn get_child_key(&self, name: String) -> Self {
        let mut index = self.to_vec();
        index.extend(name.as_bytes());
        Self::new(
            hash::sha256(Data::new(index)).as_ref()
        )
    }
    pub fn from_seed(seed: &str) -> Self {
        Self::new(hash::sha256(Data::from(seed)).as_ref())
    }
    fn is_wif(text: String) -> bool {
        Self::parse(text).unwrap().1 == "WIF".to_string()
    }
    pub fn is_valid(text: String) -> bool {
        Self::parse(text).is_ok()
    }
    pub fn randomkey(cpu_entropy: u32) -> Self {
        Self::new(keyutil::random32_byte_buffer(cpu_entropy, true))
    }
    pub fn unsafe_randomkey() -> Self {
        Self::new(keyutil::random32_byte_buffer(0, false))
    }
}
impl ToString for PublicKey {
    fn to_string(&self) -> String {
        self.to_string_with_type(None)
    }
}
impl ToString for PrivateKey {
    fn to_string(&self) -> String {
        self.to_wif()
    }
}
impl<'a> From<&'a str> for PublicKey {
    fn from(key_string: &'a str) -> Self {
        Self::from(key_string.to_string())
    }
}
impl From<String> for PublicKey {
    fn from(key_string: String) -> Self {
        match Self::from_string(key_string, Self::str_prefix(None)) {
            Ok(point) => {
                PublicKey{
                        pubkey:point
                    }
            }
            Err(err) => {
                err_print(err);
                panic!("Fail");
            }
        }
    }
}
impl<'a> From<Data<'a>> for PublicKey {
    fn from(data: Data<'a>) -> Self {
        Self::from(data.as_ref())
    }
}
impl<'a> From<&'a [u8]> for PublicKey {
    fn from(data: &'a [u8]) -> Self {
        PublicKey{
            pubkey: Self::from_data(data).unwrap()
        }
    }
}

impl<'a> From<&'a str> for PrivateKey {
    fn from(key_string: &'a str) -> Self {
        Self::from(key_string.to_string())
    }
}
impl From<String> for PrivateKey {
    fn from(key_string: String) -> Self {
        match Self::parse(key_string) {
            Ok(inner) => {
                inner.0
            }
            Err(err) => {
                err_print(err);
                panic!("Warn default Int generate");
            }
        }
    }
}
impl<'a> From<Data<'a>> for PrivateKey {
    fn from(data: Data<'a>) -> Self {
        Self::from(data.as_ref())
    }
}
impl<'a> From<&'a [u8]> for PrivateKey {
    fn from(data: &'a [u8]) -> Self {
        PrivateKey{
            pvkey: PvKey::parse_slice(data).unwrap()
        }
    }
}

impl<'a> From<&'a PrivateKey> for PublicKey {
    fn from(prikey: &'a PrivateKey) -> Self {
        PublicKey{
            pubkey: Pubkey::from_secret_key(&prikey.pvkey)
        }
    }
}
