#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let ans = 6 - a - b;

    println!("{}", ans);
}
