#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    if (4 * x - y) % 2 == 0 && 4 * x - y >= 0 && 2 * x - y <= 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
