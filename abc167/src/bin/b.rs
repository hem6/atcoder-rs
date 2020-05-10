#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        k: i64,
    }

    let ans;
    if a >= k {
        ans = k;
    } else if b >= (k - a) {
        ans = a;
    } else {
        ans = a - (k - a - b);
    }

    println!("{}", ans);
}
