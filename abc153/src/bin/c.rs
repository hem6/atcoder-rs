#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize;n],
    }

    h.sort();

    if h.len() < k {
        println!("0");
        return;
    }

    for i in 0..k {
        h.pop();
    }

    let total: usize = h.iter().sum();
    println!("{}", total);
}
