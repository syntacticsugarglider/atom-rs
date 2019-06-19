use crate::error;
use bitvec::{bitvec, cursor::BigEndian, vec::BitVec};
use failure::Error;
use std::{
    convert::{TryFrom, TryInto},
    fmt::{self, Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
#[repr(u8)]
pub enum Octant {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl From<Octant> for u8 {
    fn from(input: Octant) -> Self {
        unsafe { std::mem::transmute(input) }
    }
}

impl TryFrom<u8> for Octant {
    type Error = Error;

    fn try_from(input: u8) -> Result<Self, Self::Error> {
        if input > 7 {
            Err(error::Error::InvalidOctantIndex { index: input })?
        }
        Ok(unsafe { std::mem::transmute(input) })
    }
}

impl TryFrom<char> for Octant {
    type Error = Error;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        Ok(input.to_string().parse::<u8>()?.try_into()?)
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "@{}",
            self.octants.iter().fold(String::new(), |s, octant| {
                s + u8::from(*octant).to_string().as_str()
            })
        )
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut idents = s.chars();
        if idents.next().ok_or(error::Error::EmptyNodeAddress)? != '@' {
            Err(error::Error::InvalidNodeAddress {
                address: s.to_owned(),
            })?
        };
        Ok(Address {
            octants: idents
                .map(|ident| ident.try_into())
                .collect::<Result<Vec<_>, Error>>()?,
        })
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq)]
pub struct Address {
    octants: Vec<Octant>,
}

impl Address {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bits: BitVec = BitVec::with_capacity(self.octants.len() * 4);
        for octant in &self.octants {
            bits.push(u8::from(*octant) & 1 > 0);
            bits.push(u8::from(*octant) & 2 > 0);
            bits.push(u8::from(*octant) & 4 > 0);
            bits.push(false);
        }
        if bits.len() % 8 != 0 {
            bits.append(&mut bitvec![1, 1, 1, 1]);
        }
        bits.as_bitslice().as_slice().to_vec()
    }
    pub fn new() -> Self {
        Address::default()
    }
}

impl From<Vec<Octant>> for Address {
    fn from(octants: Vec<Octant>) -> Self {
        Address { octants }
    }
}

impl TryFrom<&[u8]> for Address {
    type Error = Error;

    fn try_from(input: &[u8]) -> Result<Self, Self::Error> {
        let mut octants = Vec::<Octant>::new();
        let bit_vec = BitVec::<BigEndian, _>::from_slice(input);
        let chunks = bit_vec.as_bitslice().chunks(4);
        let mut bad_octant = 0;
        for chunk in chunks {
            if bad_octant != 0 {
                Err(error::Error::InvalidOctantIndex { index: bad_octant })?
            }
            let mut octant = 0u8;
            if chunk.get(0).unwrap() {
                octant ^= 1;
            }
            if chunk.get(1).unwrap() {
                octant ^= 2;
            }
            if chunk.get(2).unwrap() {
                octant ^= 4;
            }
            if chunk.get(3).unwrap() {
                octant ^= 8;
            }
            if let Ok(octant) = Octant::try_from(octant) {
                octants.push(octant);
            } else {
                bad_octant = octant;
            }
        }
        Ok(octants.into())
    }
}