#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: [i32;n],
    }

    let res: i32 = (1..=100)
        .map(|i| x.iter().map(|&j| (j - i).pow(2)).sum())
        .min()
        .unwrap();

    println!("{}", res);
}
