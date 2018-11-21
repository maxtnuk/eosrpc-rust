use prelude::*;
use key::keyutil;
use key::{PublicKey, PrivateKey};
use secp256k1::{Message,RecoveryId};
use secp256k1::Signature as SeSig;

pub type ResultSignature=Result<Signature,Errortype>;

pub struct Signature{
    sig: SeSig,
    recid: RecoveryId
}
impl Signature{
    fn new(sig: SeSig,recid: RecoveryId)->Self{
        Signature{
            sig: sig,
            recid: recid
        }
    }
    pub fn verify<'a, T>(&self, data: T, pubkey: &PublicKey) -> bool
    where
        T: Into<Data<'a>>,
    {
        let data=data.into();
        let hashed=hash::sha256(data);
        self.verifyhash(hashed, pubkey)
    }
    
    pub fn verifyhash<'a, T>(&self, data: T, pubkey: &PublicKey) -> bool
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let msg=Message::parse_slice(data.as_ref()).expect("32 bytes");
        secp256k1::verify(&msg, &self.sig, &pubkey.pubkey)
    }
    pub fn recover<'a, T>(&self,data: T) -> Result<PublicKey, Errortype>
    where
        T: Into<Data<'a>>,
    {
        let data=data.into();
        let hashed=hash::sha256(data);
        self.recoverhash(hashed.as_ref())
    }
    pub fn recoverhash<'a, T>(&self,data: T) -> Result<PublicKey, Errortype>
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let msg=Message::parse_slice(data.as_ref()).expect("32 bytes");
        match secp256k1::recover(&msg,&self.sig,&self.recid) {
            Ok(val) => Ok(PublicKey{
                pubkey: val
            }),
            Err(_) => {
                Err(Errortype::MakeFail {
                    who: "PublicKey".to_string(),
                    content: None,
                })
            }
        }
    }
    pub fn sign<'a, T>(data: T, pvkey: &PrivateKey) ->Result<Self, Errortype>
    where
        T: Into<Data<'a>>,{
        let data=data.into();
        let hashed=hash::sha256(data);
        Self::signhash(hashed.as_ref(),pvkey)        
    }
    
    pub fn signhash<'a, T>(data: T, pvkey: &PrivateKey) ->Result<Self, Errortype>
    where
        T: Into<Data<'a>>,
    {
        let data = data.into();
        let msg=Message::parse_slice(data.as_ref()).expect("32 bytes");
        match secp256k1::sign(&msg, &pvkey.pvkey){
            Ok((s,r)) =>{
                Ok(Self::new(s,r))
            },
            Err(_) =>{
                Err(
                Errortype::MakeFail{
                    who: "Signature".to_string(),
                    content: None
                })
            }
        }
    }
    fn to_vec<'a>(&self) -> Data<'a>{
        let data=self.sig.serialize();
        Data::new(data.to_vec())
    }
}
impl ToString for Signature {
    fn to_string(&self) -> String {
        "SIG_K1_".to_string() +
            keyutil::check_encode(self.to_vec(), Some("K1".to_string())).as_str()
    }
}
