#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64;n],
    }

    let n: usize = n;
    let a: Vec<i64> = a;

    let mut dp: Vec<Vec<i64>> = vec![vec![-100_000_000_000_000_000; 2]; n];
    dp[0][0] = a[0];
    dp[0][1] = -a[0];

    for i in 0..n - 1 {
        dp[i + 1][0] = max(dp[i][1] - a[i + 1], dp[i][0] + a[i + 1]);
        dp[i + 1][1] = max(dp[i][1] + a[i + 1], dp[i][0] - a[i + 1]);
    }

    let ans = max(dp[n - 2][0] + a[n - 1], dp[n - 2][1] - a[n - 1]);

    println!("{}", ans);
}
