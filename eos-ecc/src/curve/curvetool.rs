use super::curvetype::{CurveParam,EcCurve};
use num::BigInt;
use num_bigint::Sign;
use prelude::*;
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use enum_map::EnumMap;

pub struct EcTools{
    scurve_params: EnumMap<CurveType,CurveParam>
}

#[derive(Clone,Debug,Enum)]
pub enum CurveType{
    Secp256K1,
    Secp256R1,
    Secp128R1,
    Secp160K1,
    Secp160R1,
    Secp192K1,
    Secp192R1,
}
#[derive(Serialize, Deserialize,Clone)]
#[serde(untagged)]
enum Argument{
    Argu {p: String,a: String,b: String,n: String,h: String,Gx: String,Gy: String}
}
#[derive(Serialize, Deserialize,Clone)]
struct Curves{
    secp128r1: Argument,
    secp160k1: Argument,
    secp160r1: Argument,
    secp192k1: Argument,
    secp192r1: Argument,
    secp256r1: Argument,
    secp256k1: Argument,
}

impl  EcTools{
    pub fn new()->Self{
        let mut file =File::open("src/curve/curves.json").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let json_data :Curves=serde_json::from_str(contents.as_str()).unwrap();
        
        let curvparams=enum_map! {
            CurveType::Secp128R1 =>{
                Self::argument_to_curveparam(CurveType::Secp128R1,json_data.clone().secp128r1)
            },
            CurveType::Secp160K1=>{
                Self::argument_to_curveparam(CurveType::Secp160K1,json_data.clone().secp160k1)
            },
            CurveType::Secp160R1=>{
                Self::argument_to_curveparam(CurveType::Secp160R1,json_data.clone().secp160r1)
            },
            CurveType::Secp192K1=>{
                Self::argument_to_curveparam(CurveType::Secp192K1,json_data.clone().secp192k1)
            },
            CurveType::Secp192R1=>{
                Self::argument_to_curveparam(CurveType::Secp192R1,json_data.clone().secp192r1)
            },
            CurveType::Secp256R1=>{
                Self::argument_to_curveparam(CurveType::Secp256R1,json_data.clone().secp256r1)
            },
            CurveType::Secp256K1=>{
                Self::argument_to_curveparam(CurveType::Secp256K1,json_data.clone().secp256k1)
            }
        };
        EcTools{
            scurve_params: curvparams
        }
    }
    fn argument_to_curveparam(ctype: CurveType, e: Argument)->CurveParam{
        match e{
            Argument::Argu{p,a,b,Gx,Gy,n,h}=>{
                 CurveParam::new(ctype,
                                EcCurve::new_from_str(p.as_str(),a.as_str(),b.as_str()),
                                 Gx.as_str(),
                                 Gy.as_str(),
                                 n.as_str(),
                                 h.as_str()
                                )
            }
        }
    }
    pub fn get_curve_param(&self,ctype: Option<CurveType>) -> CurveParam{
        let e=ctype.unwrap_or(CurveType::Secp256K1);
        self.scurve_params[e].clone()
    }
    pub fn get_byte_length(field_size: usize) -> usize{
        (field_size + 7) / 8
    }
    pub fn integer_to_vec(s: BigInt,length: usize)->Data{
        let b_data=s.to_bytes_be().1.clone();
        if length < b_data.len(){
            b_data.iter().skip(b_data.len()-length).map(|x| *x).collect()   
        }else if length > b_data.len(){
            let zeros=vec![0u8;length-b_data.len()];
            zeros.iter().chain(b_data.iter()).map(|x| *x).collect()
        }else{
            b_data
        }
    }
    pub fn vec_to_integer(d: Data)->BigInt{
        BigInt::from_bytes_be(Sign::Plus,d.as_slice())
    }
}