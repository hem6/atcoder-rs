#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        l: u64,
        r: u64,
        d: u64,
    }

    let mut ans = 0;
    for i in l..=r {
        if i % d == 0 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
