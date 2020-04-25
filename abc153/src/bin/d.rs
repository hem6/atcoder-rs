#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut h: usize,
    }

    let mut ans: usize = 0;
    let mut rate: usize = 1;
    while h > 0 {
        h /= 2;
        ans += rate;
        rate *= 2;
    }

    println!("{}", ans);
}
