use num::{BigInt, One, Zero, ToPrimitive, Integer};
use num_bigint::{RandomBits, Sign, ToBigInt};
use num::pow::pow;
use rand::{self, Rng};
use hex;
use prelude::*;
use super::curvetool::CurveType;
use super::curvetool::EcTools;
use key::primtool::{modinverse, rev_bitvec, test_bit};

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::cmp;
use std::fmt;

lazy_static!{
    static ref BIG1: BigInt = BigInt::one();
    static ref BIG2: BigInt = BigInt::one() + BigInt::one();
}
#[derive(Clone)]
pub struct CurveParam {
    ctype: CurveType,
    curve: EcCurve,
    G: Result<EcPoint, Errortype>,
    n: BigInt,
    h: BigInt,
}
impl CurveParam {
    pub fn new(
        types: CurveType,
        curve: EcCurve,
        GxInHex: &str,
        GyInHex: &str,
        nInHex: &str,
        hInHex: &str,
    ) -> Self {
        let n = BigInt::parse_bytes(nInHex.as_bytes(), 16).unwrap();
        let h = BigInt::parse_bytes(hInHex.as_bytes(), 16).unwrap();

        let before_data = "04".to_string() + GxInHex + GyInHex;
        //println!("before \ngx:{}\ngy:{}",GxInHex,GyInHex);
        CurveParam {
            ctype: types,
            curve: curve.clone(),
            G: curve.decode_point(hex::decode(before_data).unwrap()),
            n: n,
            h: h,
        }
    }
    pub fn get_curve(&self) -> EcCurve {
        self.curve.clone()
    }
    pub fn get_g(&self) -> EcPoint {
        self.G.clone().unwrap()
    }
    pub fn get_n(&self) -> BigInt {
        self.n.clone()
    }
    fn set_g<'a, S>(mut self, data: S)
    where
        S: Into<Data<'a>>,
    {
        let point = self.curve.decode_point(data);
        self.G = point;
    }
    pub fn curvetype(&self) -> CurveType {
        self.ctype.clone()
    }
}
#[derive(Eq, Clone, Debug)]
pub struct EcFieldElement {
    pub x: BigInt,
    q: BigInt,
}
impl EcFieldElement {
    fn new(x: BigInt, q: BigInt) -> Self {
        EcFieldElement { x: x, q: q }
    }
    fn square(&self) -> Self {
        EcFieldElement {
            x: (self.x.clone() * self.x.clone()).modpow(&BIG1, &self.q),
            q: self.q.clone(),
        }
    }
    fn sqrt(&self) -> Result<Self, Errortype> {
        let bitvecQ = rev_bitvec(self.q.clone());
        if !test_bit(&bitvecQ, 0) {
            return Err(Errortype::NotDone);
        }
        if test_bit(&bitvecQ, 1) {
            let exp = (self.q.clone() >> 2) + BigInt::one();
            let z = Self::new(self.x.clone().modpow(&exp, &self.q), self.q.clone());

            return if z.square() == *self {
                Ok(z)
            } else {
                Err(Errortype::NotExist("EcFieldElement".to_string()))
            };
        }
        let q_minus = &self.q - BigInt::one();
        let legendre_exponent = q_minus.clone() >> 1;
        if self.x.clone().modpow(&legendre_exponent, &self.q) == BigInt::one() {
            return Err(Errortype::NotExist("EcFieldElement".to_string()));
        }
        let u = q_minus.clone() >> 2;
        let k = (u << 1) + BigInt::one();

        let Q = self.x.clone();
        let four_q = (Q.clone() << 2).modpow(&BIG1, &self.q);
        let mut U = BigInt::one();
        let mut result_str: Option<BigInt> = None;

        while U == BigInt::one() || U == q_minus.clone() {
            let mut p = BigInt::zero().clone();

            let mut rng = rand::thread_rng();
            while p >= self.q.clone() ||
                (p.clone() * p.clone() - four_q.clone()).modpow(
                    &legendre_exponent,
                    &self.q,
                ) != q_minus || p == BigInt::zero()
            {
                p = rng.sample(RandomBits::new(self.q.bits()));
            }
            let result = self.lucas_sequence(&p, &Q, &k);
            U = result[0].clone();
            let mut V = result[1].clone();

            if (V.clone() * V.clone()).modpow(&BIG1, &self.q) == four_q {
                let bitvecV = rev_bitvec(V.clone());
                if test_bit(&bitvecV, 0) {
                    V = V.clone() + self.q.clone();
                }
                result_str = Some(V >> 1);
                break;
            }
        }
        if let Some(res) = result_str {
            Ok(Self::new(res, self.q.clone()))
        } else {
            Err(Errortype::MakeFail {
                who: "EcFieldElement".to_string(),
                content: None,
            })
        }
    }
    fn lucas_sequence(&self, P: &BigInt, Q: &BigInt, k: &BigInt) -> Vec<BigInt> {
        let n = k.bits();
        let s: usize = (k & -k).to_f64().unwrap().log2() as usize;
        let bitvecK = rev_bitvec(k.clone());
        let p = self.q.clone();

        let mut Uh = BigInt::one();
        let mut V1 = BIG2.clone();
        let mut Vh = P.clone();
        let mut Q1 = BigInt::one();
        let mut Qh = BigInt::one();

        let mut j = n - 1;
        while j >= s + 1 {
            Q1 = (Q1.clone() * Qh.clone()).mod_floor(&p);
            if test_bit(&bitvecK, j) {
                Qh = (Q1.clone() * Q.clone()).mod_floor(&p);
                Uh = (Uh.clone() * Vh.clone()).mod_floor(&p);
                V1 = (Vh.clone() * V1.clone() - P.clone() * Q1.clone()).mod_floor(&p);
                Vh = (Vh.clone() * Vh.clone() - (Qh.clone() << 1)).mod_floor(&p);
            } else {
                Qh = Q1.clone();
                Uh = (Uh.clone() * V1.clone() - Q1.clone()).mod_floor(&p);
                V1 = (V1.clone() * V1.clone() - (Qh.clone() << 1)).mod_floor(&p);
                Vh = (Vh.clone() * V1.clone() - P.clone() * Q1.clone()).mod_floor(&p);
            }
            j -= 1;
        }
        Q1 = (Q1.clone() * Qh.clone()).mod_floor(&p);
        Qh = (Q1.clone() * Q.clone()).mod_floor(&p);
        Uh = (Uh.clone() * V1.clone() - Q1.clone()).mod_floor(&p);
        V1 = (Vh.clone() * V1.clone() - P.clone() * Q1.clone()).mod_floor(&p);
        Q1 = (Q1.clone() * Qh.clone()).mod_floor(&p);

        j = 1;
        while j <= s {
            Uh = (Uh.clone() * V1.clone()).mod_floor(&p);
            V1 = (V1.clone() * V1.clone() - (Q1.clone() << 1)).mod_floor(&p);
            Q1 = (Q1.clone() * Q1.clone()).mod_floor(&p);
            j += 1;
        }
        vec![Uh, V1]
    }
    pub fn get_x(&self) -> BigInt {
        self.x.clone()
    }
}
impl Add for EcFieldElement {
    type Output = EcFieldElement;

    fn add(self, other: EcFieldElement) -> EcFieldElement {
        EcFieldElement {
            x: (self.x.clone() + other.x).mod_floor(&self.q),
            q: self.q,
        }
    }
}
impl Sub for EcFieldElement {
    type Output = EcFieldElement;

    fn sub(self, other: EcFieldElement) -> EcFieldElement {
        EcFieldElement {
            x: (self.x.clone() - other.x).mod_floor(&self.q),
            q: self.q,
        }
    }
}
impl Mul for EcFieldElement {
    type Output = EcFieldElement;

    fn mul(self, other: EcFieldElement) -> EcFieldElement {
        EcFieldElement {
            x: (self.x.clone() * other.x).mod_floor(&self.q),
            q: self.q,
        }
    }
}
impl Div for EcFieldElement {
    type Output = EcFieldElement;

    fn div(self, other: EcFieldElement) -> EcFieldElement {
        let val = modinverse(other.x, self.q.clone()).unwrap_or(BigInt::zero());
        EcFieldElement {
            x: (self.x.clone() * val).mod_floor(&self.q),
            q: self.q,
        }
    }
}
impl Neg for EcFieldElement {
    type Output = EcFieldElement;

    fn neg(self) -> EcFieldElement {
        EcFieldElement {
            x: (-self.x.clone()).mod_floor(&self.q),
            q: self.q,
        }
    }
}
impl PartialEq for EcFieldElement {
    fn eq(&self, other: &EcFieldElement) -> bool {
        self.q == other.q && self.x == other.x
    }
}
impl fmt::Display for EcFieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nx: {}\nq:{})", self.x, self.q)
    }
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct EcCurve {
    a: EcFieldElement,
    b: EcFieldElement,
    q: BigInt,
}
impl EcCurve {
    pub fn new(q: BigInt, a: BigInt, b: BigInt) -> Self {
        EcCurve {
            q: q.clone(),
            a: Self::from_big_integer(q.clone(), a),
            b: Self::from_big_integer(q.clone(), b),
        }
    }
    pub fn new_from_str(pInHex: &str, aInHex: &str, bInHex: &str) -> Self {
        let (p, a, b) = (
            BigInt::parse_bytes(pInHex.as_bytes(), 16).unwrap(),
            BigInt::parse_bytes(aInHex.as_bytes(), 16).unwrap(),
            BigInt::parse_bytes(bInHex.as_bytes(), 16).unwrap(),
        );
        EcCurve::new(p, a, b)
    }
    fn from_big_integer(q: BigInt, x: BigInt) -> EcFieldElement {
        EcFieldElement::new(x, q.clone())
    }
    fn from_self_big_integer(&self, x: BigInt) -> EcFieldElement {
        EcFieldElement::new(x, self.q.clone())
    }
    fn field_size(&self) -> usize {
        self.q.bits()
    }
    pub fn new_infinity_point(&self) -> EcPoint {
        EcPoint {
            curve: self.clone(),
            compressed: false,
            x: None,
            y: None,
        }
    }
    pub fn new_point_fromx(&self, is_odd: bool, x: BigInt) -> EcPoint {

        let alpha = (pow(x.clone(), 3) + (self.a.x.clone() * x.clone()) + self.b.x.clone())
            .mod_floor(&self.q);
        let pover_four = (self.q.clone() + BigInt::one()) >> 2;
        let beta = alpha.modpow(&pover_four, &self.q);

        let mut y = beta.clone();
        if (beta % BIG2.clone() == BigInt::zero()) ^ !is_odd {
            y = self.q.clone() - y.clone();
        }
        let x_field = self.from_self_big_integer(x);
        let y_field = self.from_self_big_integer(y);

        EcPoint::new(self.clone(), Some(x_field), Some(y_field), false)
    }
    pub fn decode_point<'a, S>(&self, encoded: S) -> Result<EcPoint, Errortype>
    where
        S: Into<Data<'a>>,
    {
        let encoded = encoded.into();
        match encoded[0] {
            0x00 => Ok(self.new_infinity_point()),
            0x02 | 0x03 => {

                let ytilde = encoded[0] & 1;

                let big_from_i = BigInt::from_bytes_be(Sign::Plus, &encoded[1..]);
                let x = EcFieldElement::new(big_from_i, self.q.clone());
                let alpha = x.clone() * (x.clone().square() + self.a.clone()) + self.b.clone();

                match alpha.sqrt() {
                    Ok(va) => {
                        let bitvecVA = rev_bitvec(va.x.clone());
                        let check = if bitvecVA.get(0).unwrap() { 1 } else { 0 };

                        if check == ytilde {
                            Ok(EcPoint::new(self.clone(), Some(x), Some(va), true))
                        } else {
                            let tmp =
                                EcFieldElement::new(self.q.clone() - va.x.clone(), self.q.clone());
                            Ok(EcPoint::new(self.clone(), Some(x), Some(tmp), true))
                        }
                    }
                    Err(err) => {
                        err_print(err);
                        Err(Errortype::DecodeFail("Point".to_string()))
                    }
                }
            }
            0x04 | 0x06 | 0x07 => {
                let len_2 = (encoded.len() - 1) / 2;
                //println!("encoded len:{}, len_2: {}",encoded.len(),len_2);

                let (xEnc, yEnc) = encoded[1..].split_at(len_2);
                let (en_x, en_y) = (hex::encode(xEnc), hex::encode(yEnc));
                //println!("after \ngx:{}\ngy:{}",,hex::encode(yEnc));
                let (big_xenc, big_yenc) = (
                    BigInt::parse_bytes(en_x.as_bytes(), 16).unwrap(),
                    BigInt::parse_bytes(en_y.as_bytes(), 16).unwrap(),
                );
                Ok(EcPoint::new(
                    self.clone(),
                    Some(EcFieldElement::new(big_xenc, self.q.clone())),
                    Some(EcFieldElement::new(big_yenc, self.q.clone())),
                    false,
                ))
            }
            _ => Err(Errortype::DecodeFail("Point".to_string())),
        }
    }
    pub fn validate(&self, q: &EcPoint) -> bool {
        !q.is_infinity() && self.is_on_curve(q)
    }
    fn is_on_curve(&self, q: &EcPoint) -> bool {
        if q.is_infinity() {
            return true;
        }
        let x = q.x.clone().unwrap();
        let y = q.y.clone().unwrap();

        let x_flag = x.x.clone() <= BigInt::zero() || x.x.clone() >= self.q;
        let y_flag = y.x.clone() <= BigInt::zero() || y.x.clone() >= self.q;
        match (x_flag, y_flag) {
            (false, false) => {

                let lhs = y.clone().square();
                let x3 = x.clone() * x.clone() * x.clone();
                let rhs = x3 + (self.a.clone() * x.clone()) + self.b.clone();

                lhs.x.mod_floor(&self.q) == rhs.x.mod_floor(&self.q)
            }
            _ => false,
        }
    }
}
#[derive(Clone, Debug, Eq)]
pub struct EcPoint {
    curve: EcCurve,
    compressed: bool,
    pub x: Option<EcFieldElement>,
    pub y: Option<EcFieldElement>,
}
impl EcPoint {
    pub fn new(
        curve: EcCurve,
        x: Option<EcFieldElement>,
        y: Option<EcFieldElement>,
        compressed: bool,
    ) -> Self {
        EcPoint {
            curve: curve,
            compressed: compressed,
            x: x,
            y: y,
        }
    }
    pub fn set_compressed(&mut self, compressed: bool) {
        self.compressed = compressed;
    }
    pub fn is_infinity(&self) -> bool {
        self.x == None && self.y == None
    }
    pub fn get_encoded<'a>(&self) -> Option<Data<'a>> {
        self.get_encoded_by_bool(self.compressed)
    }
    pub fn get_encoded_by_bool<'a>(&self, check: bool) -> Option<Data<'a>> {
        if self.is_infinity() {
            return None;
        }
        let x_clone = self.x.clone().unwrap();
        let y_clone = self.y.clone().unwrap();

        let length = EcTools::get_byte_length(x_clone.q.bits());
        if check {
            let big_y = y_clone.x.clone();
            let bitvecY = rev_bitvec(big_y);
            let pc = if bitvecY.get(0).unwrap() { 0x03 } else { 0x02 };

            let mut x: Vec<u8> = EcTools::integer_to_vec(x_clone.clone().x, length);
            x.insert(0, pc);
            Some(Data::new(x))
        } else {
            let x: Vec<u8> = EcTools::integer_to_vec(x_clone.clone().x, length);
            let y: Vec<u8> = EcTools::integer_to_vec(y_clone.clone().x, length);
            let mut result: Vec<u8> = x.iter().chain(y.iter()).map(|x| *x).collect();
            result.insert(0, 0x04);
            Some(Data::new(result))
        }
    }
    pub fn subtract(self, b: Self) -> Self {
        if b.is_infinity() {
            self
        } else {
            self.add(b.negate())
        }
    }
    pub fn add(self, b: Self) -> Self {
        if self.is_infinity() {
            return b;
        }
        if b.is_infinity() {
            return self;
        }
        let x_clone = self.x.clone().unwrap();
        let y_clone = self.y.clone().unwrap();
        if x_clone.clone() == b.x.clone().unwrap() {
            if y_clone.clone() == b.y.clone().unwrap() {
                self.twice()
            } else {
                //println!("called here");
                self.curve.new_infinity_point()
            }
        } else {
            let gamma = (b.y.unwrap().clone() - y_clone.clone()) /
                (b.x.clone().unwrap() - x_clone.clone());

            let x3 = gamma.clone().square() - x_clone.clone() - b.x.clone().unwrap();
            let y3 = gamma.clone() * (x_clone.clone() - x3.clone()) - y_clone.clone();

            EcPoint::new(self.curve, Some(x3), Some(y3), false)
        }
    }
    pub fn twice(self) -> Self {
        if self.is_infinity() {
            return self;
        }
        let x_clone = self.x.clone().unwrap();
        let y_clone = self.y.clone().unwrap();
        if y_clone.clone().x == BigInt::zero() {
            self.curve.new_infinity_point()
        } else {
            let TWO = self.curve.from_self_big_integer(2.to_bigint().unwrap());
            let THREE = self.curve.from_self_big_integer(3.to_bigint().unwrap());

            let gamma = (x_clone.clone().square() * THREE.clone() + self.curve.a.clone()) /
                (y_clone.clone() * TWO.clone());

            let x3 = gamma.clone().square() - (x_clone.clone() * TWO);
            let y3 = gamma.clone() * (x_clone.clone() - x3.clone()) - y_clone.clone();

            EcPoint::new(self.curve, Some(x3), Some(y3), self.compressed)
        }
    }
    pub fn negate(self) -> Self {
        EcPoint::new(
            self.curve,
            Some(self.x.clone().unwrap()),
            Some(-self.y.clone().unwrap()),
            self.compressed,
        )
    }
    pub fn multiply(self, k: BigInt) -> Self {
        let e = k.clone();
        let h = e.clone() * (3.to_bigint().unwrap());
        let bitvecE = rev_bitvec(e.clone());
        let bitvecH = rev_bitvec(h.clone());

        let p = self.clone();
        let mut R = self;
        let mut i = h.clone().bits() - 2;
        let R_neg = R.clone().negate();
        while i > 0 {
            R = R.twice();

            let hbit = test_bit(&bitvecH, i);
            let ebit = test_bit(&bitvecE, i);
            if ebit != hbit {
                R = R.clone().add(if hbit { p.clone() } else { R_neg.clone() });
            }
            i -= 1;
        }
        R
    }
    pub fn multiply_two(self, k: BigInt, Q: EcPoint, l: BigInt) -> Self {
        let m = cmp::max(k.bits(), l.bits());
        let p = self.clone();
        let mut R = self.curve.new_infinity_point();
        let z = self.add(Q.clone());

        let bitvecK = rev_bitvec(k.clone());
        let bitvecL = rev_bitvec(l.clone());

        let mut i = m as isize - 1 as isize;

        while i >= 0 {
            R = R.twice();

            let lbit = test_bit(&bitvecL, i as usize);
            let kbit = test_bit(&bitvecK, i as usize);
            match (kbit, lbit) {
                (true, true) => {
                    R = R.add(z.clone());
                }
                (true, false) => {
                    R = R.add(p.clone());
                }
                (false, true) => R = R.add(Q.clone()),
                _ => {}
            }
            i -= 1;
        }
        R
    }
}
impl PartialEq for EcPoint {
    fn eq(&self, other: &EcPoint) -> bool {
        if self.is_infinity() {
            return other.is_infinity();
        }
        if other.is_infinity() {
            return self.is_infinity();
        }
        self.x.clone().unwrap() == other.x.clone().unwrap() &&
            self.y.clone().unwrap() == other.y.clone().unwrap()
    }
}
