#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        g: [(Usize1, Usize1);m]
    }

    let mut count: Vec<usize> = vec![0; n];

    for (a, b) in g {
        count[a] += 1;
        count[b] += 1;
    }

    if count.iter().all(|&i| i % 2 == 0) {
        println!("YES");
    } else {
        println!("NO");
    }
}
