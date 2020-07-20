#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: u64,
        iws: [(i64, i64);n]
    }

    let mut zero: u64 = 0;
    let mut a0 = 0;
    let mut b0 = 0;
    let mut iws_count: HashMap<(i64, i64), u64> = HashMap::new();

    for (mut a, mut b) in iws {
        if a == 0 && b == 0 {
            zero += 1;
            continue;
        }
        if a == 0 {
            a0 += 1;
            continue;
        }
        if b == 0 {
            b0 += 1;
            continue;
        }

        if a < 0 && b < 0 || a > 0 && b < 0 {
            a *= -1;
            b *= -1
        }
        let gcd_num = gcd(a, b);
        a /= gcd_num;
        b /= gcd_num;

        *iws_count.entry((a, b)).or_insert(0) += 1;
    }

    let mut ans = ModInt::new(1, MOD);
    let mut seen: HashSet<(i64, i64)> = HashSet::new();

    for (&ab, &count) in &iws_count {
        let (a, b) = ab;

        if seen.get(&(a, b)).is_some() {
            continue;
        }

        let &pair_count = if a < 0 {
            seen.insert((b, -a));
            iws_count.get(&(b, -a)).unwrap_or(&0)
        } else {
            seen.insert((-b, a));
            iws_count.get(&(-b, a)).unwrap_or(&0)
        };

        ans *= ModInt::new(2, MOD).pow(count) + ModInt::new(2, MOD).pow(pair_count) - 1;
    }

    ans *= ModInt::new(2, MOD).pow(a0) + ModInt::new(2, MOD).pow(b0) - 1;
    ans += zero;

    println!("{}", (ans - 1).v);
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
