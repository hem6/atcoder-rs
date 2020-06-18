#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: u64,
        p: u64,
    }

    let ans = (a * 3 + p) / 2;
    println!("{}", ans);
}
