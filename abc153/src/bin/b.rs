#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize;n],
    }

    let total = a.iter().sum();

    if h <= total {
        println!("Yes");
    } else {
        println!("No");
    }
}
