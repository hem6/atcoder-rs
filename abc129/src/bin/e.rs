#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        l: Chars
    }
    let n = l.len();
    let mut dp: Vec<Vec<Vec<usize>>> = vec![vec![vec![0; 2]; 2]; n + 1];
    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..2 {
            for k in 0..2 {
                for d in 0..2usize {
                    let nd = l[i].to_digit(10).unwrap() as usize;
                    let mut nj = j;
                    let pattern = if d == 1 { 2 } else { 1 };

                    if j == 0 {
                        if nd < d {
                            continue;
                        } else if nd > d {
                            nj = 1;
                        }
                    }

                    dp[i + 1][nj][d] += dp[i][j][k] * pattern % 1_000_000_007;
                }
            }
        }
    }
    let ans = (dp[n][0][0] + dp[n][0][1] + dp[n][1][0] + dp[n][1][1]) % 1_000_000_007;
    println!("{}", ans);
}
