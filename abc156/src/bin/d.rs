#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n:i64,
        a:i64,
        b:i64,
    }

    let total = mod_pow(2, n, 1_000_000_007) - 1;
    let sub_a = choose(n, a, 1_000_000_007);
    let sub_b = choose(n, b, 1_000_000_007);

    let res = (total - sub_a - sub_b) % 1_000_000_007;
    let res = if res < 0 { 1_000_000_007 + res } else { res };

    println!("{}", res);
}

fn mod_pow(a: i64, n: i64, mod_n: i64) -> i64 {
    if n == 0 {
        return 1;
    }

    let mut res = mod_pow(a, n / 2, mod_n);
    res = res * res % mod_n;

    if n % 2 == 1 {
        res = res * a % mod_n;
    }

    res
}

fn inv(a: i64, mod_n: i64) -> i64 {
    mod_pow(a, mod_n - 2, mod_n)
}

fn choose(n: i64, k: i64, md: i64) -> i64 {
    let mut res1 = 1;
    let mut res2 = 1;
    let mut i = 1;
    while i <= k {
        res1 = (res1 * (n - (i - 1))) % md;
        res2 = (res2 * i) % md;
        i += 1;
    }

    res1 * inv(res2, md) % md
}
