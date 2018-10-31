use std::fmt;

#[derive(Clone)]
pub enum Errortype{
    InputWrong,
    UncompressWrong,
    MakeFail{who: String,content: Option<String>},
    ParseFail,
    ChecksumErr,
    Datalength,
    DecodeFail(String),
    NotDone,
    NotExist(String),
    InvalidKey,
    WronLength,
    InvalidSignature,
    InvalidHashtype,
    NotSame,
    NotFind
}
fn err_match(err: &Errortype)->String{
    match *err{
            Errortype::InputWrong=>{
                format!("input type wrong")
            },
            Errortype::UncompressWrong=>{
                format!("uncompressed fail")
            },
            Errortype::MakeFail{ref who,ref content}=>{
                match content{
                    Some(e) =>{
                        format!("{} make fail => {}",who,e)
                    },
                    None =>{
                        format!("{} make fail",who)
                    }
                }
            },
            Errortype::ParseFail=>{
                format!("parse fail")
            },
            Errortype::ChecksumErr=>{
                format!("invalid checksum during decode")
            },
            Errortype::Datalength=>{
                format!("wrong data length")
            },
            Errortype::DecodeFail(ref who)=>{
                format!("decode {} fail",who)
            },
            Errortype::NotDone=>{
                format!("not done yet")  
            },
            Errortype::NotExist(ref who)=>{
                format!("{} not exist",who)
            },
            Errortype::InvalidKey=>{
                format!("invalid key")
            },
            Errortype::WronLength=>{
                format!("wrong length")
            },
            Errortype::InvalidSignature=>{
                format!("Invalid signature parameter")
            },
            Errortype::InvalidHashtype=>{
                format!("Invalid hashType")
            },
            Errortype::NotSame=>{
                format!("not same")
            },
            Errortype::NotFind=>{
                format!("not find")
            }
            
    }
}
impl fmt::Debug for Errortype{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",err_match(self))
    }
}
pub fn err_print(err: Errortype){
    println!("{}",err_match(&err))
}