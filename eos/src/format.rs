use prelude::*;
use std::iter;
use num::ToPrimitive;
use num_bigint::{BigInt, ToBigInt};
use regex::Regex;
use std::str;

static CHARMAP: &str = ".12345abcdefghijklmnopqrstuvwxyz";

fn ulong(value: String,radix: Option<u32>)->BigInt{
    BigInt::parse_bytes(value.as_bytes(), radix.unwrap_or(10)).unwrap()
}

pub fn is_name(name: String)->bool{
    encode_name(name,true).is_ok()
}
pub fn encode_name_hex(name: String)->String{
    let hexname= encode_name(name,true).unwrap_or("".to_string());
    let decnum: u64 = hexname.parse().unwrap();
    //println!("encode: {}",decnum);
    format!("{:x}",decnum)
}
pub fn decode_name_hex(name: String,little_endian: bool)->String{
    let dehex=ulong(name,Some(16));
    //println!("decode: {}",dehex);
    decode_name(dehex.to_str_radix(10),little_endian)
}
pub fn encode_name(name: String,little_endian: bool)->Result<String,Errortype>{
    let charidx = |ch: char| -> Option<usize> {
        CHARMAP.chars().position(|x| x == ch)
    };
    if name.len() > 12{
        Err(Errortype::WrongLength)
    }else{
        let mut bitstr=String::new();
        for i in 0..=12{
            let i_char=name.chars().nth(i).unwrap_or(' ');
            let c = if i < name.len() { 
            let char_result= charidx(i_char);
                if char_result.is_none(){
                    return Err(Errortype::MakeFail)
                }
                char_result.unwrap()
            } else {0};
            
            let bit_len= if i < 12 {5} else {4};
            
            let bits = format!("{:b}",c);
            //println!("{}",bits);
            if bits.len() > bit_len{
                return Err(Errortype::WrongLength);
            }else{
                let t: String=iter::repeat('0').take(bit_len - bits.len()).collect();
                bitstr+=&format!("{}{}",t,bits);
            }
        }
        let value=ulong(bitstr,Some(2));
        //println!("{}",value);
        
        let mut lehex=String::new();
        let bytes= if little_endian {value.to_bytes_le().1} else {value.to_bytes_be().1};
        
        for b in bytes.iter(){
            let n = format!("{:x}",b);
            //println!("{}",n);
            lehex+=&format!("{}{}",if n.len() == 1 {"0"} else {""},n);
        }
        Ok(ulong(lehex,Some(16)).to_str_radix(10))
    }
}
pub fn decode_name(value: String,little_endian: bool)->String{
    let value=ulong(value,None);
    
    let mut behex=String::new();
    let bytes= if little_endian {value.to_bytes_le().1} else {value.to_bytes_be().1};
    
    for b in bytes.iter(){
            let n = format!("{:x}",b);
            behex+=&format!("{}{}",if n.len() == 1 {"0"} else {""},n);
    }
    let t: String=iter::repeat('0').take(16 - behex.len()).collect();
    behex+=&t;
    
    let fivebits=0x1f_u32.to_bigint().unwrap();
    let fourbits=0x0f_u32.to_bigint().unwrap();
    let bevalue=ulong(behex,Some(16));
    
    let mut result=String::new();
    let mut tmp = bevalue.clone();
    
    for i in 0..=12{
        let idx=(tmp.clone() & if i == 0 {fourbits.clone()} else {fivebits.clone()}).to_usize().unwrap();
        tmp=tmp >> (if i==0 {4}else{5});
        if let Some(c) = CHARMAP.chars().nth(idx){
            result= c.to_string() + &result;
        }
    }
    let re = Regex::new(r"\.+$").unwrap();
    re.replace_all(result.as_str(), "").to_string()
}
pub fn decimal_string(value: String)->String{
    let mut value= value;
    let re = Regex::new(r"^-").unwrap();
    let neg = re.is_match(value.as_str());
    
    if neg {
        value.remove(0);
    }
    if value.chars().nth(0).unwrap() == '.'{
        value.insert(0,'0');
    }
    
    let mut parts: Vec<String>=value.split('.').map(|x| x.to_string()).collect();
    
    if parts.len() ==2{
        let suf = Regex::new(r"0+$").unwrap();
        parts[1] = suf.replace_all(parts[1].as_str(), "").into_owned();
        if parts[1].as_str()==""{
            parts.pop();
        }
    }
    
    let rezero = Regex::new(r"^0*").unwrap();
    parts[0] = rezero.replace_all(parts[0].as_str(), "").into_owned();
    if parts[0].as_str() == ""{
        parts[0]="0".to_string();
    }
    
    if neg {"-"}else{""}.to_owned() + &parts.join(".")
}
pub fn decimal_pad(num: String,precision: Option<usize>)->String{
    let value= decimal_string(num);
    //println!("value: {}",value);
    if precision.is_none() {
        return value;
    }
    let parts: Vec<String>=value.split('.').map(|x| x.to_string()).collect();
    let precision=precision.unwrap();
    if parts.len()==1{
        if precision == 0{
            parts[0].clone()   
        }else{
            let zeros: String=iter::repeat('0').take(precision).collect();
            format!("{}.{}",parts[0],zeros)
        }
    }else{
        let pad=precision - parts[1].len();
        let zeros: String=iter::repeat('0').take(pad).collect();
        format!("{}.{}{}",parts[0],parts[1],zeros)
    }
}
pub fn decimal_imply(value: String,precision: Option<usize>)->String{
    decimal_pad(value,precision).replace(".","")
}
pub fn decimal_unimply(value: String,precision: Option<usize>)->String{
    let re = Regex::new(r"^-").unwrap();
    let neg = re.is_match(value.as_str());
    let mut value = value;
    if neg{
        value.remove(0);
    }
    
    let precision= precision.unwrap();
    let pad = precision as isize -value.len() as isize;
    
    if pad > 0{
        let zeros: String=iter::repeat('0').take(pad as usize).collect();
        value = format!("{}{}",zeros,value);
    }
    
    let dotidx= value.len()-precision;
    let (f,s)=value.split_at(dotidx);
    let result = format!("{}.{}",f.to_owned(),s.to_owned());
    if neg {"-"}else{""}.to_owned() + &decimal_pad(result,Some(precision))
}
#[derive(Debug)]
pub struct Asset{
    pub amount: Option<String>,
    pub precision: Option<usize>,
    pub symbol: String,
    pub contract: Option<String>
}
pub fn print_asset(amount: String,precision: Option<usize>,
               symbol: String,contract: String)->String{
    let mut amount=amount;
    if amount.len()!=0 && precision.is_some(){
        amount = decimal_pad(amount,precision);
    }
    let join = |a: String,b: String|{
        if a.len() == 0{
            "".to_owned()
        }else if b.len() ==0{
            "".to_owned()
        }else{
            a + &b
        }
    };
    let precision=precision.unwrap_or(0);
    if amount.len() != 0 {
        join(amount," ".to_string()) + &symbol + &join("@".to_string(),contract)
    }else{
        join(format!("{}",precision),",".to_string()) + &symbol + &join("@".to_string(),contract)
    }
}
pub fn parse_asset(text: String)->Asset{
    //let parts: Vec<String>=text.split('.').map(|x| x.to_string()).collect();
    //let amount_raw=parts[0].clone();
    let amount_match = Regex::new(r"^(-?[0-9]+(\.[0-9]+)?)( |$)").unwrap();
    let amount = 
    match amount_match.captures(text.as_str()){
        Some(e) =>{
            Some(e.get(1).unwrap().as_str().to_owned())
        },
        None =>{
            None
        }
    };
    println!("amount {:?}",amount);
    
    let precision_match = Regex::new(r"(^| )([0-9]+),([A-Z]+)(@|$)").unwrap();
    
    let precision_symbol = match precision_match.captures(text.as_str()){
        Some(e) =>{
            let r: usize =e.get(2).unwrap().as_str().parse().unwrap();
            Some(r)
        },
        None =>{None}
    };
    let precision_amount = if let Some(e) = amount.clone(){
                let ln=e.split(".").nth(1).map_or(0,|v| v.len());
                Some(ln)
    }else{
         None
    };
    
    let precision =if precision_symbol.is_some(){
        precision_symbol
    }else{
        precision_amount
    };
    
    let symbol_match = Regex::new(r"(^| |,)([A-Z]+)(@|$)").unwrap();
    let symbol=match symbol_match.captures(text.as_str()){
        Some(e)=>{
            let sym=e.get(2).unwrap().as_str();
            Some(sym.to_owned())
        },
        None =>{
            None
        }
    };
    
    let contractraw=text.split('@').nth(1).unwrap_or("");
    let contract_match = Regex::new(r"^[a-z0-5]+(\.[a-z0-5]+)*$").unwrap();
    let contract = if contract_match.is_match(contractraw){
       Some(contractraw.to_owned())
    }else{
        None
    };
    
    Asset{
        amount: amount,
        precision: precision,
        symbol: symbol.unwrap(),
        contract: contract
    }
}
