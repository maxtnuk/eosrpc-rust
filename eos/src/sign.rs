use crypto::signature::{Signature,ResultSignature};
use bincode::serialize;
use eos_api::form::all::*;
use crypto::key::PrivateKey;

pub struct SignProvider {
    pub chain_id: String,
    pub keys: chain::response::GetRequiredKeys,
    pub transaction: Transaction,
    pub abis: Vec<ABI>,
}
impl SignProvider {
    pub fn gen_sigs(&self) -> Vec<ResultSignature> {
        use crypto::to_bytes;
        let mut buf = to_bytes(self.chain_id.to_string());
        let serialtrx = serialize(&self.transaction).unwrap();
        buf.extend(serialtrx);
        buf.extend(vec![0u8; 32]);
        self.keys.require_keys.iter().fold(
            Vec::new(),
            |mut acc, x| {
                let pvkey = PrivateKey::from(x.as_str());
                acc.push(Signature::sign(buf.as_slice(), &pvkey));
                acc
            },
        )
    }
}