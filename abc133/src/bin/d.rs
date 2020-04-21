#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    }

    let total: usize = a.iter().sum();
    let x1: usize;
    let x_rest: usize = a
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 != 0)
        .map(|t| t.1)
        .sum();
    x1 = total - x_rest * 2;

    let mut ans = vec![0; n];
    ans[0] = x1;
    print!("{}", ans[0]);

    for i in 1..n {
        ans[i] = (a[i - 1] - ans[i - 1] / 2) * 2;
        print!(" {}", ans[i]);
    }
}
