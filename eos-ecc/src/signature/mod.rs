pub mod ecsignature;
pub mod ecdsa;

use prelude::*;
use num::{BigInt, ToPrimitive};
use key::{PublicKey, PrivateKey};

use curve::curvetool::CurveType;
use self::ecsignature::EcSignature;
use num_bigint::ToBigInt;
use curve::curvetool::EcTools;

use key::keyutil;

pub struct Signature {
    i: BigInt,
    sig: EcSignature,
}
impl Signature {
    fn new(r: BigInt, s: BigInt, i: BigInt, ctype: Option<CurveType>) -> Self {
        Signature {
            i: i,
            sig: EcSignature::new(r, s, ctype),
        }
    }
    fn new_raw(sig: EcSignature, i: BigInt) -> Self {
        Signature { i: i, sig: sig }
    }
    pub fn get_ecsig(&self) -> EcSignature {
        self.sig.clone()
    }
    pub fn verify<'a, T>(sig: &EcSignature, data: T, pubkey: &PublicKey) -> bool
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let hashed = hash::sha256(data);
        Self::verifyhash(sig, hashed.to_vec(), pubkey)
    }
    pub fn verifyhash<'a, T>(sig: &EcSignature, hashed: T, pubkey: &PublicKey) -> bool
    where
        T: Into<Data<'a>>,
    {
        ecdsa::verify_with_pub(hashed, sig, &pubkey)
    }
    fn recover<'a, T>(&self, data: T) -> Result<PublicKey, Errortype>
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let hashed = hash::sha256(data);
        self.recoverhash(hashed.to_vec())
    }

    fn recoverhash<'a, T>(&self, hashed: T) -> Result<PublicKey, Errortype>
    where
        T: Into<Data<'a>>,
    {
        let hashed = hashed.into();
        let e = EcTools::vec_to_integer(hashed);

        let mut i2 = self.i.clone() - 27.to_bigint().unwrap();
        i2 &= 3.to_bigint().unwrap();

        match ecdsa::recover_pubkey(self.sig.curve.clone(), e, self.sig.clone(), i2) {
            Ok(val) => Ok(PublicKey::new_raw(self.sig.curve.clone(), val)),
            Err(err) => {
                err_print(err);
                Err(Errortype::MakeFail {
                    who: "PublicKey".to_string(),
                    content: None,
                })
            }
        }
    }
    fn to_vec<'a>(&self) -> Data<'a> {
        let i_u8 = self.i.to_u8().unwrap();

        let vec_r = EcTools::integer_to_vec(self.sig.r.clone(), 32);
        let vec_s = EcTools::integer_to_vec(self.sig.s.clone(), 32);

        let mut before_data: Vec<u8> = vec_r.iter().chain(vec_s.iter()).map(|x| *x).collect();
        before_data.insert(0, i_u8);
        Data::new(before_data)
    }
    pub fn sign<'a, T>(data: T, pvkey: &PrivateKey) -> Self
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let hashed = hash::sha256(data);
        Self::signhash(hashed.to_vec(), pvkey)
    }
    pub fn signhash<'a, T>(hashed: T, pvkey: &PrivateKey) -> Self
    where
        T: Into<Data<'a>>,
    {
        let hashed = hashed.into();
        let mut nonce = 0;
        let e = EcTools::vec_to_integer(hashed.clone());

        let curve = pvkey.curveparam.clone();
        let pubkey = PublicKey::from(pvkey);
        let (i, ecdsig) = loop {
            if nonce % 10 == 0 {
                println!("WARN: {} attempts to find canonical signature", nonce);
            }
            println!("current nonce: {}", nonce);
            let in_ecdsig = ecdsa::sign(&curve, hashed.clone(), pvkey.get_mdata(), nonce);
            nonce += 1;
            let der = in_ecdsig.to_der();
            let len_r = der[3];
            let len_s = der[5 + len_r as usize];
            if len_r == 32 && len_s == 32 {
                match ecdsa::calc_pubkey_recovery_param(&curve, e.clone(), &in_ecdsig, &pubkey) {
                    Ok(val) => {
                        break (val + 31, in_ecdsig);
                    }
                    Err(err) => {
                        err_print(err);
                        continue;
                    }
                }
            }
        };
        Signature::new_raw(ecdsig, i.to_bigint().unwrap())
    }
}
impl ToString for Signature {
    fn to_string(&self) -> String {
        "SIG_K1_".to_string() +
            keyutil::check_encode(self.to_vec(), Some("K1".to_string())).as_str()
    }
}
