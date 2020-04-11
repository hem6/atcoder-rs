#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        w: i64,
        wv: [[i64;2];n],
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![w + 1; 1000 * n + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..=1000 * n {
            if (wv[i][1] as usize) <= j {
                dp[i + 1][j] = min(dp[i][j], dp[i][j - wv[i][1] as usize] + wv[i][0]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    for i in (1..=1000 * n).rev() {
        if dp[n][i] <= w {
            println!("{}", i);
            return;
        }
    }
}
