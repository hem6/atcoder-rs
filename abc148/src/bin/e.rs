#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        mut n: u64,
    }

    if n % 2 == 1 {
        println!("{}", 0);
        return;
    }

    let mut ans = 0;
    n /= 2;

    while n > 0 {
        n /= 5;
        ans += n;
    }

    println!("{}", ans);
}
