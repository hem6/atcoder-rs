#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: i64,
        v: i64,
        b: i64,
        w: i64,
        t: i64,
    }

    if (a - b).abs() <= t * (v - w) {
        println!("YES");
    } else {
        println!("NO");
    }
}
