#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        k: u64,
        s: Chars,
    }

    let h = ModInt::new(26, MOD).pow(k + s.len() as u64);
    let comb = Combination::new(k as usize + s.len(), MOD as usize);

    let mut ng = ModInt::new(0, MOD);

    for i in 0..s.len() {
        let a = ModInt::new(comb.get(k as usize + s.len(), i) as u64, MOD);
        let b = ModInt::new(25, MOD).pow(k + s.len() as u64 - i as u64);

        ng += a * b;
    }

    println!("{}", (h - ng).v);
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, Sub, SubAssign};

pub struct ModInt<T> {
    pub v: T,
    pub modulo: T,
}

impl<T> Copy for ModInt<T> where T: Copy {}
impl<T> Clone for ModInt<T>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        ModInt {
            v: self.v,
            modulo: self.modulo,
        }
    }
}

impl<T> Add<T> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy + PartialOrd,
{
    type Output = ModInt<T>;
    fn add(self, mut rhs: T) -> ModInt<T> {
        if rhs >= self.modulo {
            rhs = rhs % self.modulo;
        }
        let mut t = rhs + self.v;
        if t >= self.modulo {
            t = t - self.modulo;
        }
        ModInt {
            v: t,
            modulo: self.modulo,
        }
    }
}

impl<T> Add<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Add<T, Output = ModInt<T>>,
{
    type Output = ModInt<T>;
    fn add(self, rhs: ModInt<T>) -> ModInt<T> {
        self + rhs.v
    }
}

impl<T> Sub<T> for ModInt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Rem<Output = T> + Copy + PartialOrd,
{
    type Output = ModInt<T>;
    fn sub(self, rhs: T) -> ModInt<T> {
        let rhs = if rhs >= self.modulo {
            rhs % self.modulo
        } else {
            rhs
        };
        let value = if self.v < rhs {
            self.v + self.modulo
        } else {
            self.v
        };
        ModInt {
            v: value - rhs,
            modulo: self.modulo,
        }
    }
}

impl<T> Sub<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Sub<T, Output = ModInt<T>>,
{
    type Output = ModInt<T>;
    fn sub(self, rhs: ModInt<T>) -> ModInt<T> {
        self - rhs.v
    }
}

impl<T> AddAssign<T> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Add<T, Output = ModInt<T>>,
{
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}
impl<T> AddAssign<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Add<ModInt<T>, Output = ModInt<T>>,
{
    fn add_assign(&mut self, other: ModInt<T>) {
        *self = *self + other;
    }
}

impl<T> SubAssign<T> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Sub<T, Output = ModInt<T>>,
{
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl<T> SubAssign<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Sub<ModInt<T>, Output = ModInt<T>>,
{
    fn sub_assign(&mut self, other: ModInt<T>) {
        *self = *self - other;
    }
}

impl<T> Div<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Div<T, Output = ModInt<T>>,
{
    type Output = ModInt<T>;
    fn div(self, rhs: ModInt<T>) -> ModInt<T> {
        self / rhs.v
    }
}

impl<T> DivAssign<T> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Div<T, Output = ModInt<T>>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = *self / rhs
    }
}
impl<T> DivAssign<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Div<ModInt<T>, Output = ModInt<T>>,
{
    fn div_assign(&mut self, rhs: ModInt<T>) {
        *self = *self / rhs
    }
}

impl<T> Mul<T> for ModInt<T>
where
    T: Mul<Output = T> + Rem<Output = T> + Copy + PartialOrd,
{
    type Output = ModInt<T>;

    fn mul(self, mut rhs: T) -> ModInt<T> {
        if rhs >= self.modulo {
            rhs = rhs % self.modulo;
        }
        let t = (self.v * rhs) % self.modulo;
        ModInt {
            v: t,
            modulo: self.modulo,
        }
    }
}
impl<T> Mul<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Mul<T, Output = ModInt<T>>,
{
    type Output = ModInt<T>;
    fn mul(self, rhs: ModInt<T>) -> ModInt<T> {
        self * rhs.v
    }
}

impl<T> MulAssign<T> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Mul<T, Output = ModInt<T>>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = *self * rhs;
    }
}

impl<T> MulAssign<ModInt<T>> for ModInt<T>
where
    T: Copy,
    ModInt<T>: Mul<ModInt<T>, Output = ModInt<T>>,
{
    fn mul_assign(&mut self, rhs: ModInt<T>) {
        *self = *self * rhs;
    }
}

impl<T> ModInt<T>
where
    T: Copy,
    ModInt<T>: Mul<ModInt<T>, Output = ModInt<T>>,
{
    pub fn general_pow(self, e: u64, one: ModInt<T>) -> ModInt<T> {
        let mut result = one;
        let mut cur = self;
        let mut e = e;
        while e > 0 {
            if e & 1 == 1 {
                result *= cur;
            }
            e >>= 1;
            cur *= cur;
        }
        result
    }
}

impl Div<u64> for ModInt<u64> {
    type Output = ModInt<u64>;
    fn div(self, mut rhs: u64) -> ModInt<u64> {
        if rhs >= self.modulo {
            rhs %= self.modulo;
        }
        self * ModInt {
            v: rhs,
            modulo: self.modulo,
        }
        .pow(self.modulo - 2)
    }
}

impl ModInt<u64> {
    pub fn new(v: u64, modulo: u64) -> Self {
        Self {
            v: v % modulo,
            modulo,
        }
    }

    pub fn pow(self, e: u64) -> ModInt<u64> {
        self.general_pow(e, ModInt::new(1, self.modulo))
    }
}

pub struct Combination {
    fact: Vec<usize>,
    inv_fact: Vec<usize>,
    modulo: usize,
}

impl Combination {
    pub fn new(max: usize, modulo: usize) -> Combination {
        let mut inv = vec![0; max + 1];
        let mut fact = vec![0; max + 1];
        let mut inv_fact = vec![0; max + 1];
        inv[1] = 1;
        for i in 2..(max + 1) {
            inv[i] = inv[modulo % i] * (modulo - modulo / i) % modulo;
        }
        fact[0] = 1;
        inv_fact[0] = 1;
        for i in 0..max {
            fact[i + 1] = fact[i] * (i + 1) % modulo;
        }
        for i in 0..max {
            inv_fact[i + 1] = inv_fact[i] * inv[i + 1] % modulo;
        }
        Combination {
            fact: fact,
            inv_fact: inv_fact,
            modulo: modulo,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        assert!(x >= y);
        self.fact[x] * self.inv_fact[y] % self.modulo * self.inv_fact[x - y] % self.modulo
    }

    pub fn h(&self, n: usize, r: usize) -> usize {
        self.get(n + r - 1, r)
    }
}
