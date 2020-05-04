#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: usize,
        b: usize
    }

    let a_bit = if a > 0 { total_bit(a - 1) } else { 0 };
    let b_bit = total_bit(b);

    let ans: usize = b_bit ^ a_bit;

    println!("{}", ans);
}

fn total_bit(n: usize) -> usize {
    let mut res = (n + 1) / 2 % 2;
    if n % 2 == 0 {
        res = res ^ n;
    }
    res
}
