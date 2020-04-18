#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let gcd_num = gcd(a, b);
    let y = divisor(gcd_num);

    let ans = y.iter().filter(|&i| is_prime(*i)).count();
    println!("{}", ans + 1);
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let m = (n as f64).sqrt().floor() as i64;
    (2..=m).all(|i| n % i != 0)
}

fn divisor(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                res.push(n / i);
            }
        }
        i += 1;
    }

    res.sort();
    res
}
