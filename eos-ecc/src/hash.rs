use crypto::digest::Digest;
use crypto::sha1::Sha1;
use crypto::hmac::Hmac;
use crypto::sha2::{Sha256,Sha512};
use crypto::ripemd160::Ripemd160;
use crypto::mac::Mac;

type BoxData = Box<[u8]>;

pub fn sha1<'a>(data: &[u8])-> &'a [u8]{
    let mut hasher=Sha1::new();
    hasher.input(data);
    let mut result: BoxData=Box::new([0;32]);
    hasher.result(&mut result);
    Box::leak(result)
}

pub fn sha256<'a>(data: &[u8])-> &'a [u8]{
    let mut hasher=Sha256::new();
    hasher.input(data);
    let mut result: BoxData=Box::new([0;32]);
    hasher.result(&mut result);
    Box::leak(result)
}

pub fn sha512<'a>(data: &[u8])-> &'a [u8]{
    let mut hasher=Sha512::new();
    hasher.input(data);
    let mut result: BoxData=Box::new([0;32]);
    hasher.result(&mut result);
    Box::leak(result)
}
pub fn ripemd160<'a>(data: &[u8])-> &'a [u8]{
    let mut hasher=Ripemd160::new();
    hasher.input(data);
    let mut result: BoxData=Box::new([0;32]);
    hasher.result(&mut result);
    Box::leak(result)
}
pub fn hmac_sha256<'a>(buffer: &[u8], key: &[u8])-> &'a [u8]{
    let mut hasher=Sha256::new();
    hasher.input(buffer);
    let mut hasher=Hmac::new(hasher,key);
    let mut result: BoxData=Box::new([0;32]);
    hasher.raw_result(&mut result);
    Box::leak(result)
}