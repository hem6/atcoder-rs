#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;
use std::collections::*;
use superslice::*;

fn main() {
    input! {
        a: u64,
        b: u64,
        x: u64,
    }

    let mut ok = 0;
    let mut ng = 1_000_000_001;

    while abs(ok, ng) > 1 {
        let mid = (ok + ng) / 2;

        if calc(a, b, mid) <= x {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn calc(a: u64, b: u64, n: u64) -> u64 {
    let mut res = 0;
    res += a * n;
    let mut n = n;
    while n > 0 {
        res += b;
        n /= 10;
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
