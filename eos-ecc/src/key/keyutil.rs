use key::primtool::random_bytes;
use chrono::prelude::*;
use std::sync::Mutex;
use na::Real;
use rust_base58::base58::{ToBase58, FromBase58};
use prelude::*;


lazy_static!{
    static ref externalEntropyArray: Mutex<Vec<u8>> = Mutex::new({
        random_bytes(101)
    });
    static ref entropyPos: Mutex<u32> = Mutex::new(0);
    static ref entropyCount: Mutex<u32> = Mutex::new(0);
}

pub fn random32_byte_buffer<'a>(cpu_entropy_bits: u32, safe: bool) -> Data<'a> {

    if safe {}
    let mut hash_array: Vec<u8> = Vec::new();
    hash_array.extend(random_bytes(32));
    hash_array.extend(cpu_entropy(cpu_entropy_bits).to_vec());
    hash_array.extend(externalEntropyArray.lock().unwrap().clone());
    hash_array.extend(browser_entropy().to_vec());
    hash::sha256(Data::new(hash_array))
}

fn cpu_entropy<'a>(cpu_entropy_bits: u32) -> Data<'a> {
    let mut collected = Vec::new();
    let mut last_count: Option<f64> = None;
    let mut low_entropy_samples = 0;

    while collected.len() < cpu_entropy_bits as usize {
        let count = floating_point_count();
        if let Some(e) = last_count {
            let delta = count - e;
            if delta.abs() < 1f64 {
                low_entropy_samples += 1;
                continue;
            }

            let bits = (delta.abs().log2() + 1f64).floor() as u32;

            if bits < 4 {
                if bits < 2 {
                    low_entropy_samples += 1;
                }
                continue;
            }
            collected.push(delta as u8);
        }
        last_count = Some(count);
    }
    if low_entropy_samples > 10 {
        let pct = low_entropy_samples / cpu_entropy_bits * 100;
        println!("WARN: {} low CPU entropy re-sampled", pct);
    }
    Data::new(collected)
}
fn floating_point_count() -> f64 {
    let work_min_ms = 7;
    let d = Utc::now().timestamp_millis();
    let mut i = 0f64;
    let mut x = 0f64;
    let mut present: i64 = 0;
    while present < d + work_min_ms + 1 {
        i += 1f64;
        x = (i + x).log(f64::e()).sqrt().sin();
        present = Utc::now().timestamp_millis();
    }
    i
}
fn add_entropy(ints: Vec<u128>) {
    let mut tmpcount = entropyCount.lock().unwrap();
    let mut tmppos = entropyPos.lock().unwrap();
    let mut tmparray = externalEntropyArray.lock().unwrap();

    *tmpcount += ints.len() as u32;
    for i in ints {
        let pos = (*tmppos % 101) as usize;
        *tmppos += 1;
        let temp = tmparray[pos] as u128 + i;
        if temp > 9007199254740991 {
            tmparray[pos] = 0
        }
    }
}
fn browser_entropy<'a>() -> Data<'a> {
    let mut entropy_str = random_bytes(101);
    let cur_time = Utc::now().to_rfc2822();
    let time_data = hash::sha256(Data::from(cur_time.clone()));
    entropy_str.extend_from_slice(time_data.as_ref());

    let b = entropy_str.clone();
    entropy_str.extend(&b);
    entropy_str.extend_from_slice((" ".to_owned() + cur_time.as_str()).as_bytes());

    let mut entropy = entropy_str;
    let start_t = Utc::now().timestamp_millis();
    while Utc::now().timestamp_millis() - start_t < 25 {
        let entropy_data = hash::sha256(Data::from(entropy));
        entropy = entropy_data.as_ref().to_vec();
    }
    Data::new(entropy)
}
pub fn check_encode<'a>(keybuffer: Data<'a>, keytype: Option<String>) -> String {
    let new_check = if let Some(tp) = keytype {
        if tp == "sha256x2" {
            hash::sha256(hash::sha256(keybuffer.clone()))
        } else {
            let mut check = keybuffer.to_vec();
            check.extend_from_slice(tp.as_bytes());
            hash::ripemd160(Data::new(check))
        }
    } else {
        hash::ripemd160(keybuffer.clone())
    };
    let mut result_str = keybuffer.to_vec();
    result_str.extend_from_slice(&new_check[..4]);
    result_str.as_slice().to_base58()
}
pub fn check_decode<'a>(keybuffer: Data<'a>, keytype: Option<String>) -> ResultData {
    match keybuffer.as_ref().from_base58() {
        Ok(e) => {
            let mut buffer = e;

            let split_at = buffer.len() - 4;
            let checksum = buffer.split_off(split_at);
            let key = buffer;

            let new_check = if let Some(tp) = keytype {
                if tp == "sha256x2" {

                    hash::sha256(hash::sha256(Data::new(key.clone())))
                } else {
                    let mut check = key.to_vec();
                    check.extend_from_slice(tp.as_bytes());

                    hash::ripemd160(Data::new(check))
                }
            } else {
                hash::ripemd160(Data::new(key.to_vec()))
            };
            if checksum == new_check[0..4].to_vec() {
                Ok(Data::new(key))
            } else {
                Err(Errortype::ChecksumErr)
            }
        }
        Err(_) => Err(Errortype::DecodeFail("base 58".to_string())),
    }
}
