#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a: [i64;n],
    }

    let mut ans = 0;
    for v in a.split(|&n| n == 0) {
        ans += v.iter().sum::<i64>() / 2;
    }

    println!("{}", ans);
}
