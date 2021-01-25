#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;
extern crate byteorder;
extern crate core;

mod arith;
mod fields;
mod groups;

use fields::FieldElement;
use groups::GroupElement;
use std::ops::{Add, Sub, Mul, Neg};
use rand::{Rng, distributions::{Distribution, Standard}, thread_rng};

use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use core::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Fr(fields::Fr);

impl Fr {
    pub fn zero() -> Self {
        Fr(fields::Fr::zero())
    }
    pub fn one() -> Self {
        Fr(fields::Fr::one())
    }
    //pub fn random<R: Rng>(rng: &mut R) -> Self {        Fr(fields::Fr::random(rng))    }
    pub fn pow(&self, exp: Fr) -> Self {
        Fr(self.0.pow(exp.0))
    }
    pub fn from_str(s: &str) -> Option<Self> {
        fields::Fr::from_str(s).map(|e| Fr(e))
    }
    pub fn inverse(&self) -> Option<Self> {
        self.0.inverse().map(|e| Fr(e))
    }
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    pub fn interpret(buf: &[u8; 64]) -> Fr {
        Fr(fields::Fr::interpret(buf))
    }
}

impl Add<Fr> for Fr {
    type Output = Fr;

    fn add(self, other: Fr) -> Fr {
        Fr(self.0 + other.0)
    }
}

impl Sub<Fr> for Fr {
    type Output = Fr;

    fn sub(self, other: Fr) -> Fr {
        Fr(self.0 - other.0)
    }
}

impl Neg for Fr {
    type Output = Fr;

    fn neg(self) -> Fr {
        Fr(-self.0)
    }
}

impl Mul for Fr {
    type Output = Fr;

    fn mul(self, other: Fr) -> Fr {
        Fr(self.0 * other.0)
    }
}

impl Distribution<crate::fields::Fr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> crate::fields::Fr {
        let random_bytes: Vec<u8> = (0..64).map(|_| { rng.gen::<u8>() }).collect();
        crate::fields::Fr::interpret(&pop(random_bytes.as_ref()))
    }
}

impl Distribution<Fr> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Fr {
        let random_bytes: Vec<u8> = (0..64).map(|_| { rng.gen::<u8>() }).collect();
        Fr::interpret(&pop(random_bytes.as_ref()))
    }
}

impl fmt::Display for Fr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::convert::From<Fr> for Vec<u8> {
    fn from(elem: Fr) -> Self {
        format!("{}", elem).into_bytes()
    }
}

pub trait Group
    : 'static
    + Send
    + Sync
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Serialize
    + DeserializeOwned
    + Sized
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Neg<Output = Self>
    + Mul<Fr, Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
    fn random<R: Rng>(rng: &mut R) -> Self;
    fn is_zero(&self) -> bool;
    fn normalize(&mut self);
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct G1(groups::G1);

impl Group for G1 {
    fn zero() -> Self {
        G1(groups::G1::zero())
    }
    fn one() -> Self {
        G1(groups::G1::one())
    }
    fn random<R: Rng>(rng: &mut R) -> Self {
        G1(groups::G1::random(rng))
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add<G1> for G1 {
    type Output = G1;

    fn add(self, other: G1) -> G1 {
        G1(self.0 + other.0)
    }
}

impl Sub<G1> for G1 {
    type Output = G1;

    fn sub(self, other: G1) -> G1 {
        G1(self.0 - other.0)
    }
}

impl Neg for G1 {
    type Output = G1;

    fn neg(self) -> G1 {
        G1(-self.0)
    }
}

impl Mul<Fr> for G1 {
    type Output = G1;

    fn mul(self, other: Fr) -> G1 {
        G1(self.0 * other.0)
    }
}

impl Distribution<G1> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> G1 {
        G1(groups::G1::random(&mut thread_rng()))
    }
}

impl fmt::Display for G1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::convert::From<G1> for Vec<u8> {
    fn from(elem: G1) -> Self {
        format!("{}", elem).into_bytes()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct G2(groups::G2);

impl Group for G2 {
    fn zero() -> Self {
        G2(groups::G2::zero())
    }
    fn one() -> Self {
        G2(groups::G2::one())
    }
    fn random<R: Rng>(rng: &mut R) -> Self {
        G2(groups::G2::random(rng))
    }
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
    fn normalize(&mut self) {
        let new = match self.0.to_affine() {
            Some(a) => a,
            None => return,
        };

        self.0 = new.to_jacobian();
    }
}

impl Add<G2> for G2 {
    type Output = G2;

    fn add(self, other: G2) -> G2 {
        G2(self.0 + other.0)
    }
}

impl Sub<G2> for G2 {
    type Output = G2;

    fn sub(self, other: G2) -> G2 {
        G2(self.0 - other.0)
    }
}

impl Neg for G2 {
    type Output = G2;

    fn neg(self) -> G2 {
        G2(-self.0)
    }
}

impl Mul<Fr> for G2 {
    type Output = G2;

    fn mul(self, other: Fr) -> G2 {
        G2(self.0 * other.0)
    }
}

impl Distribution<G2> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> G2 {
        G2(groups::G2::random(&mut thread_rng()))
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[repr(C)]
pub struct Gt(fields::Fq12);

impl Gt {
    pub fn one() -> Self {
        Gt(fields::Fq12::one())
    }
    pub fn pow(&self, exp: Fr) -> Self {
        Gt(self.0.pow(exp.0))
    }
    pub fn inverse(&self) -> Self {
        Gt(self.0.inverse().unwrap())
    }
}

pub trait SerializableGt
    : 'static + Copy + Clone + Serialize + DeserializeOwned + PartialEq + Eq {
}

impl SerializableGt for Gt {}

impl Mul<Gt> for Gt {
    type Output = Gt;

    fn mul(self, other: Gt) -> Gt {
        Gt(self.0 * other.0)
    }
}

pub fn pairing(p: G1, q: G2) -> Gt {
    Gt(groups::pairing(&p.0, &q.0))
}

impl Distribution<Gt> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gt {
        pairing(rng.gen(), rng.gen())
    }
}

impl fmt::Display for Gt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::convert::From<Gt> for Vec<u8> {
    fn from(elem: Gt) -> Self {
        format!("{}", elem).into_bytes()
    }
}

fn pop(barry: &[u8]) -> [u8; 64] {
    let mut array = [0u8; 64];
    for (&x, p) in barry.iter().zip(array.iter_mut()) {
        *p = x;
    }
    array
}