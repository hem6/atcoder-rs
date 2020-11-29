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
        d: i64,
    }

    println!("{}", a * d - b * c);
}
