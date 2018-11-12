const PREFIX: &str = "EOS";

pub mod keyutil;
pub mod primtool;

use regex::Regex;
use std::any::Any;
use curve::curvetool::CurveType;
use curve::curvetype::{EcCurve, CurveParam, EcPoint};
use num::BigInt;
use num_bigint::Sign;
use prelude::*;
use std::default::Default;

type ResultParse = Result<(PrivateKey, String, String), Errortype>;
type ResultPoint = Result<EcPoint, Errortype>;

#[derive(Clone)]
pub struct PublicKey {
    pub curveparam: CurveParam,
    mdata: EcPoint,
}
impl PublicKey {
    pub fn new<'a>(
        q: &Any,
        ctype: Option<CurveType>,
        prefix: Option<&'a str>,
    ) -> Result<Self, Errortype> {
        let mut result_default: Self = Self::new_with_type(ctype);
        let curve = result_default.curveparam.get_curve();

        if let Some(string) = q.downcast_ref::<String>() {
            let in_str = Self::str_prefix(prefix);
            match Self::from_string(curve, string.clone(), in_str) {
                Ok(point) => {
                    result_default.set_point(point);
                    Ok(result_default)
                }
                Err(err) => {
                    err_print(err);
                    Err(Errortype::MakeFail {
                        who: "PublicKey".to_string(),
                        content: Some(string.clone()),
                    })
                }
            }
        } else if let Some(vecs) = q.downcast_ref::<Vec<u8>>() {
            match Self::from_data(curve, vecs.as_slice()) {
                Ok(point) => {
                    result_default.set_point(point);
                    Ok(result_default)
                }
                Err(err) => {
                    err_print(err);
                    Err(Errortype::MakeFail {
                        who: "PublicKey".to_string(),
                        content: None,
                    })
                }
            }
        } else {
            Err(Errortype::InputWrong)
        }
    }
    fn new_with_type(ctype: Option<CurveType>) -> Self {
        let mcurve = ECCTOOL.get_curve_param(ctype);
        let infity_point = mcurve.get_curve().new_infinity_point();
        PublicKey {
            mdata: infity_point,
            curveparam: mcurve,
        }
    }
    pub fn new_raw(mcurve: CurveParam, point: EcPoint) -> Self {
        PublicKey {
            mdata: point,
            curveparam: mcurve,
        }
    }
    pub fn is_valid<'a>(pubkey: String, ctype: Option<CurveType>, prefix: Option<&'a str>) -> bool {
        Self::new(&pubkey, ctype, prefix).is_ok()
    }
    fn from_string(curve: EcCurve, public_key: String, prefix: &str) -> ResultPoint {
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
            Ok(val) => Self::from_data(curve, val),
            Err(err) => {
                err_print(err);
                Err(Errortype::MakeFail {
                    who: "PublicKey".to_string(),
                    content: None,
                })
            }
        }
    }
    fn from_data<'a, S>(curve: EcCurve, data: S) -> ResultPoint
    where
        S: Into<Data<'a>>,
    {
        curve.decode_point(data)
    }
    fn set_point(&mut self, data: EcPoint) {
        self.mdata = data;
    }
    pub fn new_from_point(data: EcPoint, compressed: bool) -> Self {
        let mut result: Self = Default::default();
        result.set_point(data);
        result.mdata.set_compressed(compressed);
        result
    }
    pub fn to_string_with_type<'a>(&self, str_type: Option<&'a str>) -> String {
        Self::str_prefix(str_type).to_owned() + &keyutil::check_encode(self.to_vec(), None)
    }
    pub fn to_vec<'a>(&self) -> Data<'a> {
        self.mdata.get_encoded().unwrap()
    }
    pub fn to_uncompressed(&self) -> Result<Self, Errortype> {

        let data = self.mdata.get_encoded_by_bool(false).unwrap();
        let mut result: Self = Default::default();

        let curve = result.curveparam.get_curve();
        match Self::from_data(curve, data) {
            Ok(inner) => {
                result.set_point(inner);
                Ok(result)
            }
            Err(err) => {
                err_print(err);
                Err(Errortype::UncompressWrong)
            }
        }
    }
    pub fn get_mdata(&self) -> EcPoint {
        self.mdata.clone()
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
    mprivate_key: BigInt,
    pub curveparam: CurveParam,
}
impl PrivateKey {
    pub fn new<T>(q: T, ctype: Option<CurveType>) -> Self
    where
        T: Into<Self>,
    {
        let mut result = q.into();
        result.curveparam = ECCTOOL.get_curve_param(ctype);
        result
    }
    fn new_with_type(ctype: Option<CurveType>) -> Self {
        let mcurve = ECCTOOL.get_curve_param(ctype);
        let big_dafault: BigInt = Default::default();
        PrivateKey {
            mprivate_key: big_dafault,
            curveparam: mcurve,
        }
    }
    //{privatekey,format,keytype}
    fn parse(private_key: String) -> ResultParse {
        let re = Regex::new(r"^PVT_([A-Za-z0-9]+)_([A-Za-z0-9]+)$").expect("regex not create");

        if !re.is_match(private_key.as_str()) {
            match keyutil::check_decode(Data::from(private_key), Some("sha256x2".to_string())) {
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
            }
        } else {
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
    }
    fn set_mdata(&mut self, data: BigInt) {
        self.mprivate_key = data;
    }
    pub fn get_mdata(&self) -> BigInt {
        self.mprivate_key.clone()
    }
    fn to_wif(&self) -> String {
        let mut private_key = self.to_vec();
        private_key.insert(0, 0x80);
        keyutil::check_encode(Data::new(private_key), Some("sha256x2".to_string()))
    }
    fn get_curvetype(&self) -> Option<CurveType> {
        Some(self.curveparam.curvetype())
    }
    fn to_vec<'a>(&self) -> Vec<u8> {
        let b_data = self.mprivate_key.to_bytes_be().1.clone();
        let result = if b_data.len() < 32 {
            let zeros = vec![0u8; 32 - b_data.len()];
            zeros.iter().chain(b_data.iter()).map(|x| *x).collect()
        } else {
            b_data
        };
        result
    }
    pub fn get_shared_secret<'a>(&self, pubkey: &PublicKey) -> Data<'a> {
        let KB = pubkey.to_uncompressed().unwrap();
        let before_slice = KB.get_mdata()
            .multiply(self.mprivate_key.clone())
            .x
            .clone()
            .unwrap();
        let before = before_slice.get_x().to_bytes_be().1;
        hash::sha512(Data::from(before))
    }
    fn get_child_key(&self, name: String) -> Self {
        let mut index = self.to_vec();
        index.extend(name.as_bytes());
        Self::new(
            hash::sha256(Data::new(index)).as_ref(),
            self.get_curvetype(),
        )
    }
    pub fn from_seed(seed: &str, ctype: Option<CurveType>) -> Self {
        Self::new(hash::sha256(Data::from(seed)).as_ref(), ctype)
    }
    fn is_wif(text: String) -> bool {
        Self::parse(text).unwrap().1 == "WIF".to_string()
    }
    pub fn is_valid(text: String) -> bool {
        Self::parse(text).is_ok()
    }
    pub fn randomkey(cpu_entropy: u32, ctype: Option<CurveType>) -> Self {
        Self::new(keyutil::random32_byte_buffer(cpu_entropy, true), ctype)
    }
    pub fn unsafe_randomkey(ctype: Option<CurveType>) -> Self {
        Self::new(keyutil::random32_byte_buffer(0, false), ctype)
    }
}
impl Default for PublicKey {
    fn default() -> PublicKey {
        Self::new_with_type(None)
    }
}
impl Default for PrivateKey {
    fn default() -> PrivateKey {
        Self::new_with_type(None)
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
//ctype: none
impl<'a> From<&'a str> for PublicKey {
    fn from(key_string: &'a str) -> Self {
        Self::from(key_string.to_string())
    }
}
impl From<String> for PublicKey {
    fn from(key_string: String) -> Self {
        let mut result_default = Self::default();
        let curve = result_default.curveparam.get_curve();
        let result = Self::from_string(curve, key_string, Self::str_prefix(None));
        match result {
            Ok(point) => {
                result_default.set_point(point);
            }
            Err(err) => {
                err_print(err);
            }
        }
        result_default
    }
}
impl<'a> From<Data<'a>> for PublicKey {
    fn from(data: Data<'a>) -> Self {
        Self::from(data.as_ref())
    }
}
impl<'a> From<&'a [u8]> for PublicKey {
    fn from(data: &'a [u8]) -> Self {
        let mut result_default = Self::default();
        let curve = result_default.curveparam.get_curve();
        let result = Self::from_data(curve, data);
        match result {
            Ok(point) => {
                result_default.set_point(point);
            }
            Err(err) => {
                err_print(err);
            }
        }
        result_default
    }
}

impl<'a> From<&'a str> for PrivateKey {
    fn from(key_string: &'a str) -> Self {
        Self::from(key_string.to_string())
    }
}
impl From<String> for PrivateKey {
    fn from(key_string: String) -> Self {
        let mut result_default = Self::default();
        match Self::parse(key_string) {
            Ok(inner) => {
                result_default = inner.0;
            }
            Err(err) => {
                err_print(err);
                println!("Warn default Int generate");
            }
        }
        result_default
    }
}
impl<'a> From<Data<'a>> for PrivateKey {
    fn from(data: Data<'a>) -> Self {
        Self::from(data.as_ref())
    }
}
impl<'a> From<&'a [u8]> for PrivateKey {
    fn from(data: &'a [u8]) -> Self {
        let data = if data.len() == 33 && data[32] == 1 {
            let len = data.len() - 1;
            Ok(BigInt::from_bytes_be(Sign::Plus, &data[..len]))
        } else {
            if data.len() != 32 {
                Err(Errortype::Datalength)
            } else {
                Ok(BigInt::from_bytes_be(Sign::Plus, &data[..]))
            }
        };
        let mut result_default = Self::default();
        match data {
            Ok(inner) => {
                result_default.set_mdata(inner);
            }
            Err(err) => {
                err_print(err);
                println!("Warn default Point generate");
            }
        }
        result_default
    }
}

impl<'a> From<&'a PrivateKey> for PublicKey {
    fn from(prikey: &'a PrivateKey) -> Self {
        let q = prikey.curveparam.get_g().multiply(
            prikey.mprivate_key.clone(),
        );
        let curve = prikey.curveparam.get_curve();
        let data: EcPoint = EcPoint::new(curve.clone(), q.x, q.y, true);

        let mut result: PublicKey = Default::default();
        match data.get_encoded() {
            Some(e) => {
                let inner = PublicKey::from_data(curve, e.as_ref()).unwrap();
                result.set_point(inner);
            }
            None => {}
        }
        result
    }
}
