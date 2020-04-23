#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    for i in 1..=k {
        let blue_pattern = choose(k - 1, i - 1, MOD);
        let mut red_pattern = 0;
        if n - k >= i + 1 {
            red_pattern += choose(n - k - 1, i, MOD);
        }
        if n - k >= i {
            red_pattern += choose(n - k - 1, i - 1, MOD) * 2;
        }
        if n - k >= i - 1 && i >= 2 {
            red_pattern += choose(n - k - 1, i - 2, MOD);
        }
        if n - k == 0 && i == 1 {
            red_pattern += 1;
        }
        red_pattern %= MOD;

        println!("{}", blue_pattern * red_pattern % MOD);
    }
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
