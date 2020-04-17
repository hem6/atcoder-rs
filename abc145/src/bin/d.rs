#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    if (2 * x - y) % 3 != 0 || (2 * y - x) % 3 != 0 || (2 * x - y) < 0 || (2 * y - x) < 0 {
        println!("0");
        return;
    }

    let a = (2 * x - y) / 3;
    let b = (2 * y - x) / 3;

    let ans = mod_fact(a + b, 1_000_000_007)
        * inv(
            mod_fact(a, 1_000_000_007) * mod_fact(b, 1_000_000_007) % 1_000_000_007,
            1_000_000_007,
        )
        % 1_000_000_007;
    println!("{}", ans);
}

// aのn乗を求める。繰り返し二乗法によりO(logN)で計算できる
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

// mod_nにおける逆元(1/x)を求める。modで割り算するときは逆元をかける。
// フェルマーの小定理 x^(p-1) ≡ 1 (mod p: pは素数)より
fn inv(a: i64, mod_n: i64) -> i64 {
    mod_pow(a, mod_n - 2, mod_n)
}

fn mod_fact(a: i64, mod_n: i64) -> i64 {
    let mut count = a;
    let mut res = 1;
    while count > 0 {
        res = (res * count) % mod_n;
        count -= 1;
    }
    res
}

// nCkを求める
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
