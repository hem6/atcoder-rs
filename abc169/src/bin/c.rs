#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: u64,
        b: String,
    }

    let b = b.replace(".", "").parse::<u64>().unwrap();
    let ans = a * b / 100;
    println!("{}", ans);
}
