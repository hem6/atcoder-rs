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
        a: [u64;n],
    }

    for i in 0..n - k {
        if a[i] >= a[i + k] {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
