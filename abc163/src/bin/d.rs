#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

const MOD: u64 = 1_000_000_007;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    let mut ans = 0;

    for i in k..=n + 1 {
        let a = i * n % MOD;
        let b = i * (i - 1) % MOD;

        let ptn = if a > b {
            (a - b + 1) % MOD
        } else {
            (MOD + a - b + 1) % MOD
        };

        ans = (ans + ptn) % MOD;
    }

    println!("{}", ans);
}
