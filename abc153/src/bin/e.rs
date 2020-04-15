#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [[usize;2];n],
    }

    let mut waza: HashMap<usize, usize> = HashMap::new();
    let mut max_damage = 0;

    for v in a {
        waza.entry(v[0])
            .and_modify(|cost| {
                if v[1] < *cost {
                    *cost = v[1];
                }
            })
            .or_insert(v[1]);
        max_damage = max(max_damage, v[1]);
    }

    let mut dp: Vec<usize> = vec![100_000_001; h + max_damage];
    dp[0] = 0;

    for i in 0..h + max_damage {
        for (&damage, &cost) in &waza {
            if damage <= i {
                dp[i] = min(dp[i - damage] + cost, dp[i]);
            }
        }
    }

    let ans = dp[h..dp.len()].iter().min().unwrap();
    println!("{}", ans);
}
