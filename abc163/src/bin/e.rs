#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut a = a.iter().zip(0..n).collect::<Vec<_>>();
    a.sort();
    a.reverse();

    let mut dp = vec![vec![0; n + 1]; n + 1];

    for l in 0..n {
        for r in 0..n {
            if l + r >= n {
                break;
            }

            let (v, i) = a[l + r];

            dp[l + 1][r] = max(dp[l + 1][r], dp[l][r] + v * (l as i64 - i as i64).abs());
            dp[l][r + 1] = max(
                dp[l][r + 1],
                dp[l][r] + v * (n as i64 - 1 - r as i64 - i as i64).abs(),
            );
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans = max(ans, dp[i][n - i]);
    }

    println!("{}", ans);
}
