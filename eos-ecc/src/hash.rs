use ring::digest::{self, Context};
use ring::digest::{SHA512, SHA1, SHA256};
use ring::hmac;
use prelude::*;
use hex;

pub fn sha1<'a, T>(data: Data<'a>) -> Data<'a> {
    let mut context = Context::new(&SHA1);
    context.update(data.as_ref());
    let out = context.finish();
    Data::new(out.as_ref().to_vec())
}

pub fn sha256<'a>(data: Data<'a>) -> Data<'a> {
    let mut context = Context::new(&SHA256);
    context.update(data.as_ref());
    let out = context.finish();
    Data::new(out.as_ref().to_vec())
}

pub fn sha512<'a>(data: Data<'a>) -> Data<'a> {
    let mut context = Context::new(&SHA512);
    context.update(data.as_ref());
    let out = context.finish();
    Data::new(out.as_ref().to_vec())
}
pub fn ripemd160<'a>(data: Data<'a>) -> Data<'a> {
    use crypto::ripemd160::Ripemd160;
    use crypto::digest::Digest;

    let mut hasher = Ripemd160::new();
    hasher.input(data.as_ref());
    let mut result: Cow<'a, [u8]> = Cow::Borrowed(&[0u8; 32]);
    hasher.result(&mut result.to_mut());
    Data::new(result)
}
pub fn hmac_sha256<'a>(buffer: Data<'a>, key: Data<'a>) -> Data<'a> {
    let key = hmac::SigningKey::new(&digest::SHA256, key.as_ref());
    let sigout = hmac::sign(&key, buffer.as_ref());
    Data::new(sigout.as_ref().to_vec())
}
pub fn to_hex<'a>(data: Data<'a>) -> String {
    hex::encode(data.as_ref())
}
pub fn to_bytes<'a>(data: String) -> Vec<u8> {
    hex::decode(data.as_str()).unwrap_or(Vec::new())
}
