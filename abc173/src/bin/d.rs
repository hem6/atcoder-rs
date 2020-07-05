#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [u64;n],
    }

    a.sort();
    a.reverse();

    let mut ans = a[0];

    for i in 1..n - 1 {
        ans += a[(i + 1) / 2];
    }

    println!("{}", ans);
}
