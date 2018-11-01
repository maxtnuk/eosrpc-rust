use std::fmt;

#[derive(Clone)]
pub enum Errortype{
    WrongLength,
    MakeFail,
}
fn err_match(err: &Errortype)->String{
    match *err{
           Errortype::WrongLength=>{
               format!("wrong length")
           },
           Errortype::MakeFail=>{
               format!("make fail")
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