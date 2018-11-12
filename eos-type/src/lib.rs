use std::borrow::Cow;
use std::ops::Index;
use std::ops::{Range, RangeTo, RangeFull, RangeInclusive, RangeFrom, RangeToInclusive};

pub mod errorhandle;
pub use errorhandle::{Errortype, err_print};

pub type ResultData<'a> = Result<Data<'a>, Errortype>;
#[derive(Clone, Default)]
pub struct Data<'a> {
    pub bytes: Cow<'a, [u8]>,
}

impl<'a> Data<'a> {
    pub fn new<T>(bytes: T) -> Self
    where
        T: Into<Cow<'a, [u8]>>,
    {
        Data { bytes: bytes.into() }
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
    pub fn set<T>(&mut self, bytes: T)
    where
        T: Into<Cow<'a, [u8]>>,
    {
        self.bytes = bytes.into();
    }
}
impl<'a> AsRef<[u8]> for Data<'a> {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}
impl<'a> From<String> for Data<'a> {
    fn from(data: String) -> Self {
        Self::new(data.into_bytes())
    }
}
impl<'a> From<&'a str> for Data<'a> {
    fn from(data: &'a str) -> Self {
        let data = data.as_bytes();
        Self::new(data)
    }
}
impl<'a> From<&'a [u8]> for Data<'a> {
    fn from(data: &'a [u8]) -> Self {
        Self::new(data)
    }
}
impl<'a> From<Vec<u8>> for Data<'a> {
    fn from(data: Vec<u8>) -> Self {
        Self::new(data)
    }
}
impl<'a> Index<usize> for Data<'a> {
    type Output = u8;

    fn index(&self, idx: usize) -> &u8 {
        &self.bytes[idx]
    }
}
impl<'a> Index<Range<usize>> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Index<RangeFrom<usize>> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Index<RangeFull> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Index<RangeInclusive<usize>> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Index<RangeTo<usize>> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Index<RangeToInclusive<usize>> for Data<'a> {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &[u8] {
        &self.bytes[index]
    }
}
impl<'a> Extend<u8> for Data<'a> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let mut self_vec = self.to_vec();
        for elem in iter {
            self_vec.push(elem);
        }
        self.set(self_vec);
    }
}
