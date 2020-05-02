#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize
    }

    let x = if n >= b { n - (n % b) - 1 } else { n };
    let ans = (a as f64 * x as f64 / b as f64).floor() - a as f64 * (x as f64 / b as f64).floor();
    println!("{}", ans);
}
