#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i64,
    }

    let ans: i64 = (1..=n).filter(|&i| i % 3 != 0 && i % 5 != 0).sum();
    println!("{}", ans);
}
