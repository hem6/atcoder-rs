#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: i32,
        r: i32,
    }

    let ans = if n < 10 { r + 100 * (10 - n) } else { r };

    println!("{}", ans);
}
