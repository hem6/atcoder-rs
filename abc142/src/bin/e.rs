#![allow(unused_imports)]
use itertools::Itertools;
use num::integer::gcd;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        keys: [(i64, [Usize1]);m],
    }

    let ptn = 1 << n;

    let keys = keys
        .into_iter()
        .map(|(price, boxes)| {
            let boxes = boxes.into_iter().fold(0, |acc, x| acc | 1 << x);
            (price, boxes)
        })
        .collect::<Vec<(i64, usize)>>();

    let mut dp: Vec<Vec<i64>> = vec![vec![12 * 100_000 + 1; ptn + 1]; m + 1];
    dp[0][0] = 0;

    for i in 0..m {
        let (price, boxes) = &keys[i];
        for j in 0..ptn {
            dp[i + 1][j | boxes] = min(dp[i + 1][j | boxes], dp[i][j] + price);
            dp[i + 1][j] = min(dp[i + 1][j], dp[i][j]);
        }
    }

    let mut ans = std::i64::MAX;
    for i in 0..=m {
        ans = min(dp[i][ptn - 1], ans);
    }

    if ans >= 12 * 100_000 + 1 {
        ans = -1;
    }

    println!("{}", ans);
}
