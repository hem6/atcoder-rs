#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        h: [i64;n]
    }

    let mut dp = vec![1_000_000_000; n];
    dp[0] = 0;

    for i in 0..n - 1 {
        dp[i + 1] = min(dp[i + 1], dp[i] + (h[i] - h[i + 1]).abs());
        if i < n - 2 {
            dp[i + 2] = min(dp[i + 2], dp[i] + (h[i] - h[i + 2]).abs());
        }
    }

    print!("{}", dp[n - 1]);
}
