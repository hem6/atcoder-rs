#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }

    if r1 == r2 && c1 == c2 {
        println!("0");
    } else if r1 + c1 == r2 + c2 || r1 - c1 == r2 - c2 || (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        println!("1");
    } else if (r1 + c1) % 2 == (r2 + c2) % 2
        || (r1 + c1 - r2 - c2).abs() <= 3
        || (r1 - c1 - r2 + c2).abs() <= 3
        || (r1 - r2).abs() + (c1 - c2).abs() <= 6
    {
        println!("2");
    } else {
        println!("3");
    }
}
