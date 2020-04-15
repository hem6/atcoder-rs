#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;

    for i in 0..=60 {
        let count = a.iter().filter(|&v| v >> i & 1u64 == 1u64).count();
        let sum = ((1 << i) % 1_000_000_007) * count % 1_000_000_007;
        let sum = sum * (n - count) % 1_000_000_007;
        ans = (ans + sum) % 1_000_000_007;
    }

    println!("{}", ans);
}
