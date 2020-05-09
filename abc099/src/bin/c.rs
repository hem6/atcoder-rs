#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: usize,
    }

    let mut pow6: Vec<usize> = (1..=6).map(|i| 6usize.pow(i)).collect();
    let mut pow9: Vec<usize> = (1..=5).map(|i| 9usize.pow(i)).collect();

    pow6.append(&mut pow9);
    pow6.append(&mut vec![1]);

    let pow = pow6;

    let mut dp: Vec<usize> = vec![n; 200_000];
    dp[0] = 0;

    for i in 0..n {
        for &j in &pow {
            dp[i + j] = min(dp[i + j], dp[i] + 1);
        }
    }

    println!("{}", dp[n]);
}
