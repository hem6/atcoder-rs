#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64;n],
    }

    let mut ng = 0;
    let mut ok = 1_000_000_001;

    while abs(ok, ng) > 1 {
        let mid = (ok + ng) / 2;

        if calc(&a, mid) <= k {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn calc(a: &Vec<u64>, length: u64) -> u64 {
    let mut res = 0;

    for &v in a {
        res += (v + length - 1) / length - 1;
    }

    res
}

fn abs(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}
