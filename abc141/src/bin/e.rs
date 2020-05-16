#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        s:Chars,
    }

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for i in 0..n {
        for j in 0..n {
            if s[i] == s[j] {
                dp[i + 1][j + 1] = max(dp[i + 1][j + 1], dp[i][j] + 1);
            }
        }
    }

    let mut ans = 0;
    for i in 0..n + 1 {
        for j in 0..n + 1 {
            if i < j && dp[i][j] <= j - i {
                ans = max(ans, dp[i][j]);
            }
        }
    }

    println!("{}", ans);
}
