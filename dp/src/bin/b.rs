#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64;n],
    }

    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;

    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                dp[i + j] = min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }

    println!("{}", dp[n - 1]);
}
