extern crate crypto;
extern crate rust_base58;
extern crate exonum_sodiumoxide;
extern crate chrono;
extern crate nalgebra as na;
extern crate regex;
extern crate num;
extern crate num_bigint;
extern crate bit_vec;
extern crate rand;
extern crate byteorder;
extern crate hex;
extern crate rayon;

extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_map;

use errorhandle::Errortype;
use curve::curvetool::EcTools;

pub type Data= Vec<u8>;
pub type ResultData=Result<Data,Errortype>;

#[allow(dead_code)]
pub mod hash;
pub mod errorhandle;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod key;
#[allow(non_snake_case)]
#[allow(dead_code)]
pub mod curve;
pub mod prelude;
#[allow(dead_code)]
pub mod signature;
#[cfg(test)]
mod test;

lazy_static!{
    pub static ref ECCTOOL: EcTools = EcTools::new();
}