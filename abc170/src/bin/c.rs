#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: i64,
        n: usize,
        p: [i64;n]
    }

    let mut ans = -1;

    for i in -1..=101 {
        if p.contains(&i) {
            continue;
        }

        if (x - i).abs() < (x - ans).abs() {
            ans = i;
        }
    }

    println!("{}", ans);
}
