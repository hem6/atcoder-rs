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
        mut p: [u64;n],
    }

    p.sort();

    let ans = &p[0..k].iter().sum::<u64>();

    println!("{}", ans);
}
