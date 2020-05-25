#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut dishes: [(usize, u64);n]
    }

    let mut dp: Vec<Vec<Vec<u64>>> = vec![vec![vec![0; 2]; t]; n + 1];

    for i in 0..n {
        for j in 0..t {
            for k in 0..2 {
                let (a, b) = dishes[i];
                dp[i + 1][j][k] = max(dp[i + 1][j][k], dp[i][j][k]);

                if a <= j {
                    dp[i + 1][j][k] = max(dp[i + 1][j][k], dp[i][j - a][k] + b);
                }

                if k == 0 {
                    dp[i + 1][j][1] = max(dp[i + 1][j][1], dp[i][j][k] + b);
                }
            }
        }
    }

    let max_score = dp[n][t - 1][1];

    println!("{}", max_score);
}
