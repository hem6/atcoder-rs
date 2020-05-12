#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        q: usize,
        qq: [[usize;2];q]
    }

    let sums: Vec<usize> = (0..=100000)
        .scan(0, |st, n| {
            if is_prime(n) && is_prime((n + 1) / 2) {
                *st += 1;
            }

            Some(*st)
        })
        .collect();

    for query in qq {
        println!("{}", sums[query[1]] - sums[query[0] - 1]);
    }
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    let m = (n as f64).sqrt().floor() as i64;
    (2..=m).all(|i| n % i != 0)
}
