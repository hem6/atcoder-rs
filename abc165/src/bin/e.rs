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
    }

    let mut shift = 0;

    for i in 0..m {
        let l = m - i;
        let r = m + 1 + i;
        if n % 2 == 0 && shift == 0 && i * 2 >= n - r + l - 1 {
            shift = 1;
        }
        println!("{} {}", l, r + shift);
    }
}
