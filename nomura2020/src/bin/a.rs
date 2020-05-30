#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h1: u64,
        m1: u64,
        h2: u64,
        m2: u64,
        k: u64,
    }

    let a = h1 * 60 + m1;
    let b = h2 * 60 + m2;
    let c = if a < b { b - a } else { b + 24 * 60 - a };
    let ans = c - k;
    println!("{}", ans);
}
