#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    let mut ans = 0;

    for i in 0..n {
        if i % 2 == 0 && a[i] % 2 == 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
