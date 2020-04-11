#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        p: [[u32;3];n]
    }

    let mut dp: Vec<Vec<u32>> = vec![vec![0; 3]; n + 1];

    for i in 0..n {
        dp[i + 1][0] = max(dp[i][1] + p[i][0], dp[i][2] + p[i][0]);
        dp[i + 1][1] = max(dp[i][0] + p[i][1], dp[i][2] + p[i][1]);
        dp[i + 1][2] = max(dp[i][0] + p[i][2], dp[i][1] + p[i][2]);
    }

    println!("{}", max(dp[n][0], max(dp[n][1], dp[n][2])));
}
