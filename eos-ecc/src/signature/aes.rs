use key::{PrivateKey, PublicKey};

use crypto::aes::{KeySize, cbc_decryptor, cbc_encryptor};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{RefReadBuffer, RefWriteBuffer};
use exonum_sodiumoxide::randombytes as rb;
use byteorder::{ByteOrder, WriteBytesExt, LittleEndian};

use prelude::*;
use std::sync::Mutex;

lazy_static!{
    static ref unique_nonce_entropy:i64 = {
        let b=rb::randombytes(2);
        ((b[0] as u16) << 8 | b[1] as u16) as i64
    };
    static ref count:Mutex<i64>= Mutex::new(0);
}

pub fn encrypt<'a>(
    private_key: &PrivateKey,
    public_key: &PublicKey,
    message: Data<'a>,
    nonce: Option<i64>,
) -> Data<'a> {
    let in_nonce = match nonce {
        Some(val) => val,
        None => unique_nonce(),
    };
    crypt(private_key, public_key, message, in_nonce, None)
        .0
        .unwrap()
}
pub fn decrypt<'a>(
    private_key: &PrivateKey,
    public_key: &PublicKey,
    message: Data<'a>,
    nonce: i64,
    checksum: u32,
) -> Data<'a> {
    crypt(private_key, public_key, message, nonce, Some(checksum))
        .0
        .unwrap()
}
//(message,checksum)
fn crypt<'a>(
    private_key: &PrivateKey,
    public_key: &PublicKey,
    message: Data<'a>,
    nonce: i64,
    checksum: Option<u32>,
) -> (ResultData<'a>, u32) {

    let nonce = nonce as u64;
    let s = private_key.get_shared_secret(public_key);
    let mut ebuf: Vec<u8> = Vec::new();
    ebuf.write_u64::<LittleEndian>(nonce).unwrap();
    for i in s.to_vec().iter() {
        ebuf.write_u8(*i).unwrap();
    }
    let encryption_key = hash::sha512(Data::new(ebuf.clone()));

    let (key, iv) = ebuf.as_slice().split_at(32);
    let check_bytes = &hash::sha256(encryption_key)[..4];
    let check = LittleEndian::read_u32(&check_bytes);
    let crypt_message = if let Some(e) = checksum {
        if check != e {
            Err(Errortype::InvalidKey)
        } else {
            Ok(crypto_decrypt(
                message,
                Data::new(key.to_vec()),
                Data::new(iv.to_vec()),
            ))
        }
    } else {
        Ok(crypto_encrypt(
            message,
            Data::new(key.to_vec()),
            Data::new(iv.to_vec()),
        ))
    };
    (crypt_message, check)
}
fn crypto_encrypt<'a>(message: Data<'a>, key: Data<'a>, iv: Data<'a>) -> Data<'a> {
    let mut cipher = cbc_encryptor(KeySize::KeySize256, key.as_ref(), iv.as_ref(), NoPadding);
    let mut output: Box<[u8]> = Box::new([0u8; 32]);

    {
        let mut output_buffer = RefWriteBuffer::new(&mut output);
        let mut input_buffer = RefReadBuffer::new(message.as_ref());
        cipher
            .encrypt(&mut input_buffer, &mut output_buffer, false)
            .unwrap();
    }
    let result: Vec<u8> = output.iter().chain(output.iter()).map(|x| *x).collect();
    Data::new(result)
}
fn crypto_decrypt<'a>(message: Data<'a>, key: Data<'a>, iv: Data<'a>) -> Data<'a> {
    let mut decipher = cbc_decryptor(KeySize::KeySize256, key.as_ref(), iv.as_ref(), NoPadding);
    let mut output: Box<[u8]> = Box::new([0u8; 32]);

    {
        let mut output_buffer = RefWriteBuffer::new(&mut output);
        let mut input_buffer = RefReadBuffer::new(message.as_ref());
        decipher
            .decrypt(&mut input_buffer, &mut output_buffer, false)
            .unwrap();
    }
    let result: Vec<u8> = output.iter().chain(output.iter()).map(|x| *x).collect();
    Data::new(result)
}
// need some configure
fn unique_nonce() -> i64 {
    use chrono::prelude::*;
    let mut tmpcount = count.lock().unwrap();
    let start_t = Utc::now().timestamp_millis();
    *tmpcount += 1;
    let entropy = (*tmpcount + *unique_nonce_entropy) % 0xFFFF;
    (start_t << 16) | entropy
}
