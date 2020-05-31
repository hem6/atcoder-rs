#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        xs: [(u64, u64);n]
    }

    let mut xs_from: Vec<u64> = xs.iter().map(|v| v.0).collect();
    xs_from.sort();

    let mut xs_to: Vec<u64> = xs.iter().map(|v| v.1).collect();
    xs_to.sort();

    let ans = if n % 2 == 1 {
        med(&xs_to) - med(&xs_from) + 1.0
    } else {
        (med(&xs_to) - med(&xs_from)) * 2.0 + 1.0
    };

    println!("{}", ans as u64);
}

fn med(v: &Vec<u64>) -> f64 {
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2] as f64
    } else {
        (v[n / 2] + v[n / 2 - 1]) as f64 / 2.0
    }
}
