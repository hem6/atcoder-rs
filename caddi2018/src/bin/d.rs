#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [u64;n],
    }

    if a.into_iter().all(|i| i % 2 == 0) {
        println!("second");
    } else {
        println!("first");
    }
}
