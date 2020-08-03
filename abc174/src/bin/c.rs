#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        k: u64
    }

    let mut ans = 0;
    let mut current = 0;

    for _ in 0..k {
        current = current * 10 + 7;
        current %= k;
        ans += 1;

        if current == 0 {
            println!("{}", ans);
            return;
        }
    }

    println!("-1");
}
