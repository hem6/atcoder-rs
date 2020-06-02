#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:i64,
        m:i64,
        a:[i64;n-1]
    }

    let current: i64 = a.iter().sum();
    let last = m * n as i64 - current;
    if last > k {
        println!("-1");
    } else {
        println!("{}", max(last, 0));
    }
}
