use curve::curvetype::CurveParam;
use curve::curvetool::{EcTools, CurveType};
use prelude::*;
use num::BigInt;
use num_bigint::Sign;
use byteorder::WriteBytesExt;
use rayon::prelude::*;

#[derive(Clone)]
pub struct EcSignature {
    pub r: BigInt,
    pub s: BigInt,
    pub curve: CurveParam,
}
type ResultEcSignature = Result<EcSignature, Errortype>;
impl EcSignature {
    pub fn new(r: BigInt, s: BigInt, ctype: Option<CurveType>) -> Self {
        Self::from_raw(r, s, ECCTOOL.get_curve_param(ctype))
    }
    pub fn from_raw(r: BigInt, s: BigInt, curve: CurveParam) -> Self {
        EcSignature {
            r: r,
            s: s,
            curve: curve,
        }
    }
    pub fn to_compact(&self, i: u8, compressed: bool) -> Data {
        let mut i = i;
        if compressed {
            i += 4;
        }
        i += 27;
        let mut ebuf: Data = Vec::new();
        ebuf.write_u8(i).unwrap();
        let vec_r = EcTools::integer_to_vec(self.r.clone(), 32);
        let vec_s = EcTools::integer_to_vec(self.s.clone(), 32);

        ebuf.par_iter()
            .chain(vec_r.par_iter())
            .chain(vec_s.par_iter())
            .map(|x| *x)
            .collect()
    }
    pub fn to_der(&self) -> Data {
        let mut vec_r = EcTools::integer_to_vec(self.r.clone(), 32);
        let mut vec_s = EcTools::integer_to_vec(self.s.clone(), 32);

        let (r_len, s_len) = (vec_r.len() as u8, vec_s.len() as u8);

        vec_r.insert(0, r_len);
        vec_r.insert(0, 0x02);
        vec_s.insert(0, s_len);
        vec_s.insert(0, 0x02);

        let mut before_data: Data = vec_r
            .par_iter()
            .chain(vec_s.par_iter())
            .map(|x| *x)
            .collect();
        let before_len = before_data.len() as u8;
        before_data.insert(0, before_len);
        before_data.insert(0, 0x30);
        before_data
    }
    pub fn to_script_signature(&self, hashtype: u8) -> Data {
        let mut hashtypebuffer: Data = Vec::new();
        hashtypebuffer.write_u8(hashtype).unwrap();
        self.to_der()
            .par_iter()
            .chain(hashtypebuffer.par_iter())
            .map(|x| *x)
            .collect()
    }
    //{compressed,i,signature}
    pub fn parse_compact(
        buffer: Data,
        ctype: Option<CurveType>,
    ) -> Result<(bool, u8, Self), Errortype> {
        if buffer.len() != 65 {
            Err(Errortype::WronLength)
        } else {
            let mut i = buffer[0] - 27;
            if i == i & 7 {
                Err(Errortype::InvalidSignature)
            } else {
                let compressed = if i & 4 == 0 { false } else { true };
                i = i & 3;

                let rb = &buffer[1..33];
                let sb = &buffer[33..];

                let (r, s) = (
                    BigInt::from_bytes_be(Sign::Plus, rb),
                    BigInt::from_bytes_be(Sign::Plus, sb),
                );
                Ok((compressed, i, Self::new(r, s, ctype)))
            }
        }
    }
    pub fn from_der(buffer: Data, ctype: Option<CurveType>) -> Self {
        assert!(buffer[0] == 0x30, "Not der sequence");
        assert!(
            buffer[1] == buffer.len() as u8 - 2,
            "Invalid sequence length"
        );
        assert!(buffer[2] == 0x02, "Expected a DER integer");

        let rlen = buffer[3];
        assert!(rlen > 0, "R length is zero");
        let mut offset = 4 + rlen;
        assert!(
            buffer[offset as usize] == 0x02,
            "Expected a DER integer (2)"
        );
        let slen = buffer[offset as usize + 1];
        assert!(slen > 0, "S length is zero");

        let rb = &buffer[4..offset as usize];
        let sb = &buffer[offset as usize..];
        offset += 2 + slen;

        assert!(offset as usize == buffer.len(), "Invalid der encoding");
        let (r, s) = (
            BigInt::from_bytes_be(Sign::Plus, rb),
            BigInt::from_bytes_be(Sign::Plus, sb),
        );
        Self::new(r, s, ctype)
    }
    pub fn parse_script_signature(
        buffer: Data,
        ctype: Option<CurveType>,
    ) -> Result<(Self, u8), Errortype> {
        let hashtype = buffer[buffer.len() - 1];
        let hashtypemod = hashtype & !0x80;

        if hashtypemod > 0x00 && hashtypemod < 0x04 {
            Ok((EcSignature::from_der(buffer, ctype), hashtype))
        } else {
            Err(Errortype::InvalidHashtype)
        }
    }
}
impl Default for EcSignature {
    fn default() -> EcSignature {
        let mcurve = ECCTOOL.get_curve_param(None);
        let r_dafault: BigInt = Default::default();
        let s_dafault: BigInt = Default::default();
        Self::from_raw(r_dafault, s_dafault, mcurve)
    }
}
