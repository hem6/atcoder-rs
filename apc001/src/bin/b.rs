#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        a: [isize;n],
        b: [isize;n],
    }

    let mut a_count = 0;
    let mut b_count = 0;

    for i in 0..n {
        if a[i] > b[i] {
            b_count += a[i] - b[i];
        } else {
            a_count += (b[i] - a[i]) / 2
        }
    }

    if a_count < b_count {
        println!("No");
    } else {
        println!("Yes");
    }
}
