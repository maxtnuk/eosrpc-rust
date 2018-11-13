use prelude::*;
use signature::ecsignature::EcSignature;
use curve::curvetype::EcPoint;
use curve::curvetype::CurveParam;
use curve::curvetool::EcTools;

use num_bigint::ToBigInt;
use num::{BigInt, Integer};
use num::{Zero, One};
use key::PublicKey;
use key::primtool::modinverse;

use std::sync::mpsc::channel;

pub fn deterministic_generate_k<'a, F>(
    curve: &CurveParam,
    hashed: Data<'a>,
    d: BigInt,
    mut checksig: F,
    nonce: i64,
) -> Result<BigInt, Errortype>
where
    F: FnMut(BigInt) -> bool,
{
    let in_hash = if nonce != 0 {
        let zeros = vec![0u8; nonce as usize];
        let mut hashed = hashed;
        hashed.extend(zeros);
        hash::sha256(hashed).to_vec()
    } else {
        hashed.to_vec()
    };
    if in_hash.len() != 32 {
        Err(Errortype::WrongLength)
    } else {

        let x = EcTools::integer_to_vec(d, 32);
        let mut k = Data::new(vec![0u8; 32]);
        let mut v = Data::new(vec![1u8; 32]);

        let mut d_step_buffer = v.to_vec();
        d_step_buffer.extend_from_slice(&[0u8]);
        d_step_buffer.extend(x.clone());
        d_step_buffer.extend(in_hash.clone());
        k = hash::hmac_sha256(Data::new(d_step_buffer), k);

        v = hash::hmac_sha256(v, k.clone());

        let mut f_step_buffer = v.to_vec();
        f_step_buffer.extend_from_slice(&[1u8]);
        f_step_buffer.extend(x.clone());
        f_step_buffer.extend(in_hash.clone());
        k = hash::hmac_sha256(Data::new(f_step_buffer), k);

        v = hash::hmac_sha256(v.clone(), k.clone());
        v = hash::hmac_sha256(v.clone(), k.clone());

        let mut t = EcTools::vec_to_integer(v.clone());

        while t <= BigInt::zero() || t >= curve.get_n() || !checksig(t.clone()) {
            let mut tmp = v.to_vec();
            tmp.extend_from_slice(&[0]);
            k = hash::hmac_sha256(Data::new(tmp), k);
            v = hash::hmac_sha256(v.clone(), k.clone());

            v = hash::hmac_sha256(v.clone(), k.clone());

            t = EcTools::vec_to_integer(v.clone());
        }
        Ok(t)
    }
}

pub fn sign<'a, T>(curve: &CurveParam, hashed: T, d: BigInt, nonce: i64) -> EcSignature
where
    T: Into<Data<'a>>,
{
    let hashed = hashed.into();
    let e = EcTools::vec_to_integer(hashed.clone());
    let (n, g) = (curve.get_n(), curve.get_g());

    let mut r = BigInt::default();
    let mut s = BigInt::default();
    let _k = deterministic_generate_k(
        curve,
        hashed,
        d.clone(),
        |b| {
            let q = g.clone().multiply(b.clone());

            if q.is_infinity() {
                return false;
            }
            r = q.x.unwrap().x.mod_floor(&n);
            if r == BigInt::zero() {
                false
            } else {
                s = (modinverse(b.clone(), n.clone()).unwrap() *
                         (e.clone() + d.clone() * r.clone()))
                    .mod_floor(&n);
                s != BigInt::zero()
            }
        },
        nonce,
    );
    //println!("{},{}",r,s);
    let n_over_two = n.clone() >> 1;

    if s > n_over_two {
        s = n - s.clone();
    }
    EcSignature::new(r, s, Some(curve.curvetype()))
}
pub fn verify_raw(curve: &CurveParam, e: BigInt, sig: &EcSignature, q: EcPoint) -> bool {
    let n = curve.get_n();
    let g = curve.get_g();

    let s_clone = sig.s.clone();
    let r_clone = sig.r.clone();

    let r_flag = r_clone <= BigInt::zero() || r_clone >= n;
    let s_flag = s_clone <= BigInt::zero() || s_clone >= n;
    match (r_flag, s_flag) {
        (false, false) => {
            let c = modinverse(s_clone.clone(), n.clone()).unwrap();

            let u1 = (e.clone() * c.clone()).mod_floor(&n);
            let u2 = (r_clone.clone() * c.clone()).mod_floor(&n);

            let r = g.multiply_two(u1, q.clone(), u2);

            if r.is_infinity() {
                false
            } else {
                r.x.unwrap().x.mod_floor(&n) == r_clone
            }
        }
        _ => false,
    }
}
pub fn verify<'a, T>(curve: &CurveParam, hashed: T, sig: &EcSignature, q: EcPoint) -> bool
where
    T: Into<Data<'a>>,
{
    let hashed = hashed.into();
    let e = EcTools::vec_to_integer(hashed);
    verify_raw(curve, e, sig, q)
}
pub fn verify_with_pub<'a, T>(hashed: T, sig: &EcSignature, pubkey: &PublicKey) -> bool
where
    T: Into<Data<'a>>,
{
    let hashed = hashed.into();
    let e = EcTools::vec_to_integer(hashed);
    verify_raw(&pubkey.curveparam, e, sig, pubkey.get_mdata())
}
#[allow(non_snake_case)]
pub fn recover_pubkey(
    curve: CurveParam,
    e: BigInt,
    sig: EcSignature,
    i: BigInt,
) -> Result<EcPoint, Errortype> {
    if i.clone() & 3.to_bigint().unwrap() != i.clone() {
        return Err(Errortype::NotSame);
    }
    let n = curve.get_n();
    let g = curve.get_g();

    let s_clone = sig.s.clone();
    let r_clone = sig.r.clone();

    let r_flag = r_clone <= BigInt::zero() || r_clone >= n;
    let s_flag = s_clone <= BigInt::zero() || s_clone >= n;
    match (r_flag, s_flag) {
        (false, false) => {
            let is_odd = i.clone() & BigInt::one() == BigInt::one();

            let is_second = i.clone() >> 1 == BigInt::one();
            //need fix
            let x = if is_second {
                r_clone.clone() + n.clone()
            } else {
                r_clone.clone()
            };
            let R = curve.get_curve().new_point_fromx(is_odd, x);

            let nr = R.clone().multiply(n.clone());
            if nr.is_infinity() {
                //println!("nR is not a valid curve point");
                return Err(Errortype::MakeFail {
                    who: "Recovery pubkey".to_string(),
                    content: None,
                });
            }
            // Compute -e from e
            let eneg = {
                (-e).mod_floor(&n)
            };

            let rinv = modinverse(r_clone.clone(), n.clone()).unwrap();

            let q = R.multiply_two(s_clone, g, eneg).multiply(rinv);
            if curve.get_curve().validate(&q) {
                Ok(q)
            } else {
                Err(Errortype::MakeFail {
                    who: "Recovery pubkey".to_string(),
                    content: None,
                })
            }
        }
        _ => {
            Err(Errortype::MakeFail {
                who: "Recovery pubkey".to_string(),
                content: None,
            })
        }
    }
}
pub fn calc_pubkey_recovery_param(
    curve: &CurveParam,
    e: BigInt,
    sig: &EcSignature,
    pubkey: &PublicKey,
) -> Result<u32, Errortype> {
    use rayon::prelude::*;

    let not_find = 5;
    let (sender, receiver) = channel();
    let q = pubkey.get_mdata();
    //println!("{:?}", pubkey.to_string());
    (0..4 as u32).into_par_iter().for_each_with(
        sender,
        |s, x: u32| {
            //println!("current i: {}", x);
            let q_prime = recover_pubkey(
                curve.clone(),
                e.clone(),
                sig.clone(),
                x.to_bigint().unwrap(),
            );

            let return_val: u32 = match q_prime {
                Ok(val) => if val == q { x } else { not_find },
                Err(err) => {
                    //err_print(err);
                    not_find
                }
            };
            s.send(return_val).unwrap();
        },
    );

    let mut result = Err(Errortype::NotFind);
    let mut idxs = Vec::with_capacity(4);
    for _ in 0..4 {
        idxs.push(receiver.recv().unwrap());
    }
    for i in idxs.iter() {
        if not_find != *i {
            result = Ok(*i);
            break;
        }
    }
    result
}
/*
let handles: Vec<_> = (0..4).map(|idx| {
let tx=sender.clone();
let t_curve=curve.clone();
let t_sig=sig.clone();
let q = pubkey.get_mdata();
let t_e=e.clone();
thread::spawn(move || {
    //println!("current i: {}",idx);

    let q_prime=recover_pubkey(t_curve,t_e,t_sig,idx.to_bigint().unwrap());

    let return_val: u32=match q_prime{
        Ok(val)=>{
            //println!("val \nx: {}\n y:{}",val.clone().x.unwrap(),val.clone().y.unwrap());
            //println!("q \nx: {}\n y:{}",q.clone().x.unwrap(),q.clone().y.unwrap());
            if val==q{
                idx
            }else{
                not_find
            }
        },
        Err(err)=>{
            err_print(err);
            not_find
        }
    };
    tx.send(return_val).unwrap();
})
}).collect();
for handle in handles {
handle.join().expect("Unable to join");
}
*/
