#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize;n],
    }

    let mut total: usize = p[0..k].iter().sum();
    let mut max_total = total;
    for i in 0..n - k {
        total += p[k + i];
        total -= p[i];
        max_total = max(max_total, total);
    }

    println!("{}", (max_total + k) as f64 / 2.0);
}
