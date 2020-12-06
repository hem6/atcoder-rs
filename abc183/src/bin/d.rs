#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        w: i64,
        stp: [(usize, usize, i64);n],
    }

    let mut a = vec![0; 200_001];

    for (s, t, p) in stp {
        a[s] += p;
        a[t] -= p;
    }

    let mut l = 0;
    for x in a {
        if w - l < x {
            println!("No");
            return;
        }
        l += x;
    }

    println!("Yes");
}
