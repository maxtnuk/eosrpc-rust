pub mod aes;
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
    pub fn verify(sig: &EcSignature, data: Data, pubkey: &PublicKey) -> bool {
        let hashed = hash::sha256(data.as_slice()).to_vec();
        Self::verifyhash(sig, hashed, pubkey)
    }
    pub fn verifyhash(sig: &EcSignature, hashed: Data, pubkey: &PublicKey) -> bool {
        ecdsa::verify_with_pub(hashed, sig, &pubkey)
    }
    fn recover(&self, data: Data) -> Result<PublicKey, Errortype> {
        let hashed = hash::sha256(data.as_slice()).to_vec();
        self.recoverhash(hashed)
    }

    fn recoverhash(&self, hashed: Data) -> Result<PublicKey, Errortype> {
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
    fn to_vec(&self) -> Data {
        let i_u8 = self.i.to_u8().unwrap();

        let vec_r = EcTools::integer_to_vec(self.sig.r.clone(), 32);
        let vec_s = EcTools::integer_to_vec(self.sig.s.clone(), 32);

        let mut before_data: Data = vec_r.iter().chain(vec_s.iter()).map(|x| *x).collect();
        before_data.insert(0, i_u8);
        before_data
    }
    pub fn sign<T>(data: T, pvkey: &PrivateKey) -> Self
    where
        T: Into<Data>,
    {
        let data = data.into();
        let hashed = hash::sha256(data.as_slice());
        Self::signhash(hashed, pvkey)
    }
    pub fn signhash<T>(hashed: T, pvkey: &PrivateKey) -> Self
    where
        T: Into<Data>,
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
