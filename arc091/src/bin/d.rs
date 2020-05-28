#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut ans = 0;

    for i in k + 1..=n {
        ans += (n / i) * max(0, i - k);
        ans += max(0, (n % i) - k + 1);
        ans -= if k == 0 { 1 } else { 0 };
    }

    println!("{}", ans);
}
