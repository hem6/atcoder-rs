#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        w: usize,
        wv: [[i64;2];n],
    }

    let mut dp: Vec<Vec<i64>> = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        for j in 0..=w {
            if (wv[i][0] as usize) <= j {
                dp[i + 1][j] = max(dp[i][j], dp[i][j - wv[i][0] as usize] + wv[i][1]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[n][w]);
}
