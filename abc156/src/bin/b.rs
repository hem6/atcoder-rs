#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: u32,
        k: u32,
    }

    let mut ans = 0;

    while n > 0 {
        n /= k;
        ans += 1;
    }

    println!("{}", ans);
}
