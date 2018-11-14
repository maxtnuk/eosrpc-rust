use num::Integer;
use num_bigint::{RandomBits, BigInt};
use rand::thread_rng;
use rand::distributions::Distribution;
use bit_vec::BitVec;

pub fn egcd<T: Clone + Integer>(a: T, b: T) -> (T, T, T) {
    assert!(a < b);
    if a == T::zero() {
        (b, T::zero(), T::one())
    } else {
        let (g, x, y) = egcd(b.clone() % a.clone(), a.clone());
        (g, y - (b.clone() / a.clone()) * x.clone(), x)
    }
}
pub fn modinverse<T: Clone + Integer>(a: T, m: T) -> Option<T> {
    let (g, x, _) = egcd(a.clone(), m.clone());
    if g != T::one() {
        None
    } else {
        Some(x % m.clone())
    }
}
pub fn rev_bitvec(data: BigInt) -> BitVec {
    let bitvecVA = BitVec::from_bytes(data.clone().to_bytes_be().1.as_slice());
    bitvecVA.iter().rev().collect()
}
pub fn test_bit(o: &BitVec, idx: usize) -> bool {
    match o.get(idx) {
        Some(e) => e,
        None => false,
    }
}
pub fn random_bytes(bytes: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let inner: BigInt = RandomBits::new(bytes * 8).sample(&mut rng);
    inner.to_bytes_be().1
}
